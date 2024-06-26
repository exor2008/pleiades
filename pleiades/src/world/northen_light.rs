use super::OnDirection;
use crate::apds9960::Direction;
use crate::color::{Color, ColorGradient};
use crate::led_matrix::WritableMatrix;
use crate::perlin;
use crate::world::utils::CooldownValue;
use crate::world::{Flush, Tick};
use core::iter::Sum;
use embassy_time::{Duration, Ticker};
use heapless::Vec;
use perlin::rand_float;
use pleiades_macro_derive::Flush;
use smart_leds::RGB8;

const PATTERNS_COOLDOWNL: u8 = 1;
const PATTERNS_MAX: usize = 9;
const PATTERNS_MIN: usize = 2;
const PATTERNS_INIT: usize = 6;

#[derive(Flush)]
pub struct NorthenLight<'led, Led: WritableMatrix, const C: usize, const L: usize, const N: usize> {
    led: &'led mut Led,
    colormap: ColorGradient<C>,
    ticker: Ticker,
    patterns: Vec<Pattern<L, C, N>, PATTERNS_MAX>,
    curr_n_patterns: CooldownValue<PATTERNS_COOLDOWNL, PATTERNS_MIN, PATTERNS_MAX>,
    t: usize,
    last_spawn: isize,
}

impl<'led, Led: WritableMatrix, const C: usize, const L: usize, const N: usize>
    NorthenLight<'led, Led, C, L, N>
{
    pub fn new(led: &'led mut Led) -> Self {
        let ticker = Ticker::every(Duration::from_millis(20));
        let colormap = NorthenLight::<'led, Led, C, L, N>::get_colormap();
        let patterns: Vec<Pattern<L, C, N>, PATTERNS_MAX> = Vec::new();
        let curr_n_patterns = CooldownValue::new(PATTERNS_INIT);

        Self {
            led,
            colormap,
            ticker,
            patterns,
            curr_n_patterns,
            t: 0,
            last_spawn: -1000,
        }
    }
}

impl<'led, Led: WritableMatrix, const C: usize, const L: usize, const N: usize> Tick
    for NorthenLight<'led, Led, C, L, N>
{
    async fn tick(&mut self) {
        self.led.clear();

        self.spawn_patterns();
        self.process_patterns();
        let sum_pattern: Pattern<L, C, N> = self.patterns.iter().sum();

        for index in 0..N {
            let temperature = sum_pattern.data()[index];
            let color = self.colormap.get(temperature);
            self.led.write_straight(index, color);
        }

        self.remove_obsolete_patterns();

        self.t = self.t.wrapping_add(1);
        self.ticker.next().await;
    }
}

impl<'led, Led: WritableMatrix, const C: usize, const L: usize, const N: usize>
    NorthenLight<'led, Led, C, L, N>
{
    fn spawn_patterns(&mut self) {
        let time_till_last_spawn = self.t as isize - self.last_spawn;
        let is_limit = self.patterns.len() >= *self.curr_n_patterns.value();
        let spawn_cooldown = 100 - *self.curr_n_patterns.value() as isize * 9;

        if !self.patterns.is_full() && !is_limit && time_till_last_spawn > spawn_cooldown {
            let cutoff = rand_float(0.5, 0.55);
            let lifetime = 250 + rand_float(-75.0, 75.0) as usize;
            let pattern: Pattern<L, C, N> = Pattern::new(self.t, cutoff, lifetime);
            self.patterns.push(pattern).unwrap();
            self.last_spawn = self.t as isize;
        }
    }

    fn process_patterns(&mut self) {
        self.patterns.iter_mut().for_each(|pattern| pattern.tick());
    }

    fn remove_obsolete_patterns(&mut self) {
        self.patterns
            .retain(|pattern| pattern.t <= pattern.lifetime);
    }

    fn get_colormap() -> ColorGradient<C> {
        let mut colormap = ColorGradient::new();

        colormap.add_color(Color::new(0.0, RGB8::new(0, 0, 0)));
        colormap.add_color(Color::new(0.1, RGB8::new(0, 0, 0)));
        colormap.add_color(Color::new(0.25, RGB8::new(10, 30, 60)));
        colormap.add_color(Color::new(0.5, RGB8::new(2, 237, 80)));
        colormap.add_color(Color::new(0.75, RGB8::new(108, 134, 206)));
        colormap.add_color(Color::new(1.01, RGB8::new(70, 30, 100)));

        colormap
    }
}

impl<'led, Led: WritableMatrix, const C: usize, const L: usize, const N: usize> OnDirection
    for NorthenLight<'led, Led, C, L, N>
{
    fn on_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.curr_n_patterns.up();
            }
            Direction::Down => {
                self.curr_n_patterns.down();
            }
        }
    }
}

#[derive(Debug)]
struct Pattern<const L: usize, const C: usize, const N: usize> {
    data: [f32; N],
    lifetime: usize,
    t: usize,
}

impl<const L: usize, const C: usize, const N: usize> Pattern<L, C, N> {
    pub fn new(t: usize, cutoff: f32, lifetime: usize) -> Self {
        let noise = perlin::PerlinNoise::new();
        let data = Self::fill(noise, t, cutoff);
        Self {
            data,
            lifetime,
            t: 0,
        }
    }

    fn index(x: usize, y: usize) -> usize {
        match x % 2 == 0 {
            true => x * L + y,
            false => x * L + (L - y) - 1,
        }
    }

    fn fill(noise: perlin::PerlinNoise, t: usize, cutoff: f32) -> [f32; N] {
        let mut data = [f32::default(); N];
        let shift = rand_float(0.1, 0.8);

        for x in 0..C {
            for y in 0..L {
                // Generate noise for northen light
                let xx = (x.wrapping_add(t)) as f32 / 5.0;
                let yy = (y.wrapping_add(t)) as f32 / 5.0;
                // let zz = t as f32;

                let noise = noise.get2d([xx, yy]);
                let noise = noise - cutoff;
                let noise = if noise <= 0.0 {
                    0.0
                } else {
                    (noise + shift).min(1.0)
                };
                let index = Self::index(x, y);
                data[index] = noise;
            }
        }
        data
    }

    fn data(&self) -> &[f32; N] {
        &self.data
    }

    fn impact(&self, index: usize) -> f32 {
        self.data[index] * self.coef()
    }

    fn coef(&self) -> f32 {
        let t = self.t as f32;
        let lifetime = self.lifetime as f32;

        match t {
            t if t <= lifetime * 0.5 => t / (lifetime * 0.5),
            t => 1.0 - (t - lifetime * 0.5) / (lifetime * 0.5),
        }
    }

    fn tick(&mut self) {
        self.t += 1;
    }
}

impl<'a, const L: usize, const C: usize, const N: usize> Sum<&'a Pattern<L, C, N>>
    for Pattern<L, C, N>
{
    fn sum<I: Iterator<Item = &'a Pattern<L, C, N>>>(iter: I) -> Self {
        let mut sum_data: [f32; N] = [0.0; N];

        for item in iter {
            for (i, sum_data) in sum_data.iter_mut().enumerate().take(N) {
                *sum_data += item.impact(i);
            }
        }
        for sum_data in sum_data.iter_mut().take(N) {
            *sum_data += sum_data.clamp(0.0, 1.0)
        }
        Pattern::from(sum_data)
    }
}

impl<const L: usize, const C: usize, const N: usize> From<[f32; N]> for Pattern<L, C, N> {
    fn from(data: [f32; N]) -> Self {
        Self {
            data,
            lifetime: 0,
            t: 0,
        }
    }
}
