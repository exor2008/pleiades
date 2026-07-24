use crate::buffer::Point;
use crate::world::Tick;
use crate::world::utils::CooldownValue;
use embassy_time::{Duration, Ticker};
use ledlab::buffer::Buffer;
use ledlab::color::Color;
use ledlab::color::ColorGradient;
use ledlab::perlin;
use ledlab::utils::Direction;
use ledlab::world::GetTicker;
use ledlab::world::OnDirection;
use smart_leds::RGB8;

const HUE_COOLDOWN: u8 = 0;
const HUE_MIN: usize = 0;
const HUE_MAX: usize = 75;

pub struct Solid<const C: usize, const L: usize, const N: usize> {
    colormap: ColorGradient<8>,
    hue: CooldownValue<HUE_COOLDOWN, HUE_MIN, HUE_MAX>,
    ticker: Ticker,
    t: usize,
}

impl<const C: usize, const L: usize, const N: usize> Solid<C, L, N> {
    pub fn new() -> Self {
        let colormap = Solid::<C, L, N>::get_colormap();
        let init_hue = perlin::rand_uint(HUE_MIN as u32, HUE_MAX as u32) as usize;
        let hue = CooldownValue::new(init_hue);
        let ticker = Ticker::every(Duration::from_millis(50));

        Self {
            colormap,
            hue,
            ticker,
            t: 0,
        }
    }

    fn get_colormap() -> ColorGradient<8> {
        let mut colormap = ColorGradient::new();
        colormap.add_color(Color::new(0.0, RGB8::new(255, 0, 255)));
        colormap.add_color(Color::new(0.15, RGB8::new(255, 0, 0)));
        colormap.add_color(Color::new(0.3, RGB8::new(255, 255, 0)));
        colormap.add_color(Color::new(0.45, RGB8::new(0, 255, 0)));
        colormap.add_color(Color::new(0.6, RGB8::new(0, 255, 255)));
        colormap.add_color(Color::new(0.75, RGB8::new(0, 0, 255)));
        colormap.add_color(Color::new(0.9, RGB8::new(255, 255, 255)));
        colormap.add_color(Color::new(1.01, RGB8::new(255, 255, 255)));

        colormap
    }
}

impl<const C: usize, const L: usize, const N: usize> Default for Solid<C, L, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B, const C: usize, const L: usize, const N: usize> Tick<Point, B, N> for Solid<C, L, N>
where
    B: Buffer<Point, N>,
{
    fn tick(&mut self, buffer: &mut B) {
        buffer.clear();

        let value = (*self.hue.value()) as f32 / HUE_MAX as f32;
        let color = self.colormap.get(value);
        buffer.bg(color);

        self.t = self.t.wrapping_add(1);
    }
}

impl<const C: usize, const L: usize, const N: usize> GetTicker for Solid<C, L, N> {
    fn get_ticker(&mut self) -> &mut Ticker {
        &mut self.ticker
    }
}

impl<const C: usize, const L: usize, const N: usize> OnDirection for Solid<C, L, N> {
    fn on_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.hue.up();
            }
            Direction::Down => {
                self.hue.down();
            }
        }
    }
}
