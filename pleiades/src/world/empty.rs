use super::OnDirection;
use crate::apds9960::Direction;
use crate::buffer::Buffer;
use crate::world::{Flush, Tick};
use embassy_time::{Duration, Ticker};
use pleiades_macro_derive::Flush;

#[derive(Flush)]
pub struct Empty<'led, Led: Buffer> {
    led: &'led mut Led,
    ticker: Ticker,
}

impl<'led, Led: Buffer> Empty<'led, Led> {
    pub fn new(led: &'led mut Led) -> Self {
        let ticker = Ticker::every(Duration::from_millis(50));

        Empty { led, ticker }
    }
}

impl<'led, Led: Buffer> Tick for Empty<'led, Led> {
    async fn tick(&mut self) {
        self.led.clear();
        self.ticker.next().await;
    }
}

impl<'led, Led: Buffer> OnDirection for Empty<'led, Led> {
    fn on_direction(&mut self, _direction: Direction) {}
}
