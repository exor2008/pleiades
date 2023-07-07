use crate::ws2812::Ws2812;
use embassy_rp::pio::Instance;
pub use fire::{Flush, Tick};

pub mod fire;

pub enum World<'a, P, const S: usize, const L: usize, const C: usize, const N: usize>
where
    P: Instance,
{
    Fire(fire::Fire<'a, P, S, L, C, N>),
}

impl<'a, P, const S: usize, const L: usize, const C: usize, const N: usize> World<'a, P, S, L, C, N>
where
    P: Instance,
{
    pub fn fire_from(ws: Ws2812<'a, P, S, N>) -> Self {
        let fire = fire::Fire::from(ws);
        World::Fire(fire)
    }
}

impl<'a, P, const S: usize, const L: usize, const C: usize, const N: usize>
    Into<Ws2812<'a, P, S, N>> for World<'a, P, S, L, C, N>
where
    P: Instance,
{
    fn into(self) -> Ws2812<'a, P, S, N> {
        match self {
            Self::Fire(fire) => fire.into(),
        }
    }
}