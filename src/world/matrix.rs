use crate::buffer::Point;
use crate::world::Tick;
use core::marker::PhantomData;
use embassy_time::{Duration, Ticker};
use heapless::Vec;
use ledlab::cooldown::CooldownValue;
use ledlab::{
    buffer::Buffer,
    color::{Color, ColorGradient},
    perlin,
    utils::Direction,
    world::{GetTicker, OnDirection},
};
use smart_leds::RGB8;

const SPARKS_COOLDOWN: u8 = 3;
const SPARKS_MIN_CHANCE: usize = 2;
const SPARKS_MAX_CHANCE: usize = 5;
const MAX_LETTERS: usize = 512;

pub struct Matrix<const C: usize, const L: usize, const N: usize> {
    colormap: ColorGradient<C>,
    letters: Vec<Letters, MAX_LETTERS>,
    ticker: Ticker,
    rnd_col: Vec<usize, C>,
    spawn_chance: CooldownValue<SPARKS_COOLDOWN, SPARKS_MIN_CHANCE, SPARKS_MAX_CHANCE>,
    t: usize,
}

impl<const C: usize, const L: usize, const N: usize> Matrix<C, L, N> {
    pub fn new() -> Self {
        let ticker = Ticker::every(Duration::from_millis(30));
        let mut colormap = ColorGradient::new();
        let spawn_chance = CooldownValue::new(2);
        let letters: Vec<Letters, MAX_LETTERS> = Vec::new();
        let rnd_col: Vec<usize, C> = Vec::new();

        colormap.add_color(Color::new(0.0, RGB8::new(0, 0, 0)));
        colormap.add_color(Color::new(0.8, RGB8::new(5, 50, 5)));
        colormap.add_color(Color::new(1.01, RGB8::new(50, 150, 50)));

        Self {
            colormap,
            letters,
            ticker,
            spawn_chance,
            rnd_col,
            t: 0,
        }
    }
}

impl<const C: usize, const L: usize, const N: usize> Default for Matrix<C, L, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B, const C: usize, const L: usize, const N: usize> Tick<Point, B, N> for Matrix<C, L, N>
where
    B: Buffer<Point, N>,
{
    fn tick(&mut self, buffer: &mut B) {
        buffer.clear();

        self.spawn_letters();
        self.process_letters();
        self.remove_letters();

        self.letters.iter_mut().for_each(|letter| match letter {
            Letters::Falling(l) => {
                let color = self.colormap.get(l.temperature);
                buffer.write(Point { x: l.x, y: l.y }, color);
            }
            Letters::Stationary(l) => {
                let color = self.colormap.get(l.temperature);
                buffer.write(Point { x: l.x, y: l.y }, color);
            }
        });

        self.t = self.t.wrapping_add(1);
    }
}

impl<const C: usize, const L: usize, const N: usize> GetTicker for Matrix<C, L, N> {
    fn get_ticker(&mut self) -> &mut Ticker {
        &mut self.ticker
    }
}

impl<const C: usize, const L: usize, const N: usize> OnDirection for Matrix<C, L, N> {
    fn on_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.spawn_chance.up(),
            Direction::Down => self.spawn_chance.down(),
        }
    }
}

impl<const C: usize, const L: usize, const N: usize> Matrix<C, L, N> {
    fn spawn_letters(&mut self) {
        let chance = perlin::rand_float(0.0, 1.0);
        let prob = 1.0 - *self.spawn_chance.value() as f32 / 10.0;

        if !self.letters.is_full() && chance >= prob {
            let x: usize = self.next_rnd_column();

            let cool_rate = perlin::rand_float(0.005, 0.015);
            let temperature = perlin::rand_float(0.8, 1.0);
            let move_after = perlin::rand_uint(1, 12) as usize;

            let letter = Letters::new_falling(x, 0, move_after, temperature, cool_rate);
            if self.letters.push(letter).is_err() {
                defmt::error!("Pushing letter in full vector while spawning.");
            }
        }
    }

    fn process_letters(&mut self) {
        let mut tmp_letters: Vec<Letters, N> = Vec::new();

        self.letters.iter_mut().for_each(|letter| match letter {
            Letters::Falling(l) => {
                if l.down() {
                    let letter =
                        Letters::new_stationary(l.x, l.y - 1, l.temperature - 0.2, l.cool_rate);
                    if tmp_letters.push(letter).is_err() {
                        defmt::error!("Pushing letter in full tmp vector.")
                    }
                }
            }

            Letters::Stationary(l) => {
                l.cool();
            }
        });

        if MAX_LETTERS - self.letters.len() >= tmp_letters.len() {
            self.letters.extend(tmp_letters);
        } else {
            defmt::error!(
                "Pushing letter in full vector. Vector len: {}, new letters len: {}",
                self.letters.len(),
                tmp_letters.len()
            )
        }
    }

    fn remove_letters(&mut self) {
        self.letters.retain(|letter| match letter {
            Letters::Falling(l) => l.y < L,
            Letters::Stationary(l) => l.temperature > 0.0,
        });
    }

    fn next_rnd_column(&mut self) -> usize {
        if self.rnd_col.is_empty() {
            self.rnd_col = (0..C).collect();
            perlin::shuffle(&mut self.rnd_col);
        }
        self.rnd_col.remove(self.rnd_col.len() - 1)
    }
}

struct Falling;
struct Stationary;

enum Letters {
    Falling(Letter<Falling>),
    Stationary(Letter<Stationary>),
}

impl Letters {
    fn new_stationary(x: usize, y: usize, temperature: f32, cool_rate: f32) -> Self {
        Self::Stationary(Letter {
            x,
            y,
            move_after: Default::default(),
            move_after_init: Default::default(),
            temperature,
            cool_rate,
            star_type: Default::default(),
        })
    }

    fn new_falling(
        x: usize,
        y: usize,
        move_after: usize,
        temperature: f32,
        cool_rate: f32,
    ) -> Self {
        Self::Falling(Letter {
            x,
            y,
            move_after,
            move_after_init: move_after,
            temperature,
            cool_rate,
            star_type: Default::default(),
        })
    }
}

#[derive(Debug)]
struct Letter<LetterType> {
    x: usize,
    y: usize,
    move_after: usize,
    move_after_init: usize,
    temperature: f32,
    cool_rate: f32,
    star_type: PhantomData<LetterType>,
}

impl Letter<Falling> {
    fn down(&mut self) -> bool {
        match self.move_after == 0 {
            true => {
                self.y += 1;
                self.move_after = self.move_after_init;
                self.temperature += perlin::rand_float(-0.2, 0.2);
                self.temperature = self.temperature.clamp(0.8, 1.0);
                true
            }
            false => {
                self.move_after -= 1;
                false
            }
        }
    }
}

impl Letter<Stationary> {
    fn cool(&mut self) {
        match self.move_after == 0 {
            true => {
                self.temperature -= self.cool_rate;
                self.temperature = self.temperature.max(0.0);

                self.move_after = self.move_after_init;
            }
            false => {
                self.move_after -= 1;
            }
        }
    }
}
