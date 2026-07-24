use crate::buffer::Point;
use crate::world::Tick;
use embassy_time::{Duration, Ticker};
use ledlab::{
    buffer::Buffer,
    utils::Direction,
    world::{GetTicker, OnDirection},
};

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

impl<B, const N: usize> Tick<Point, B, N> for Empty
where
    B: Buffer<Point, N>,
{
    fn tick(&mut self, buffer: &mut B) {
        buffer.clear();
    }
}

impl GetTicker for Empty {
    fn get_ticker(&mut self) -> &mut Ticker {
        &mut self.ticker
    }
}

impl OnDirection for Empty {
    fn on_direction(&mut self, _direction: Direction) {}
}
