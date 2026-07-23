use crate::buffer::Point;
use crate::world::Tick;
use embassy_time::{Duration, Ticker};
use ledlab::{buffer::Buffer, utils::Direction};
use smart_leds::RGB8;

pub struct Empty {
    ticker: Ticker,
}

impl Empty {
    pub fn new() -> Self {
        let ticker = Ticker::every(Duration::from_millis(50));

        Empty { ticker }
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> Tick<RGB8, Point, B> for Empty
where
    B: Buffer<RGB8, Point>,
{
    type Ticker = Ticker;

    fn tick(&mut self, buffer: &mut B) {
        buffer.clear();
    }

    fn ticker(&mut self) -> &mut Self::Ticker {
        &mut self.ticker
    }

    fn on_direction(&mut self, _direction: Direction) {}
}
