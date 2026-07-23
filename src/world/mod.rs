use crate::{
    buffer::Point,
    world::{
        empty::Empty, fire::Fire, matrix::Matrix, northern_light::NorthernLight, solid::Solid,
        starry_night::StarryNight, voronoi::Voronoi,
    },
};
use embassy_time::Ticker;
use ledlab::{
    buffer::Buffer,
    world::{Tick, World},
};
use smart_leds::RGB8;

pub mod empty;
pub mod fire;
pub mod matrix;
pub mod northern_light;
pub mod solid;
pub mod starry_night;
pub mod utils;
pub mod voronoi;

#[allow(clippy::large_enum_variant)]
pub enum WorldEnum<const C: usize, const L: usize, const N: usize> {
    Empty(Empty),
    Fire(Fire<C, L>),
    NorthernLight(NorthernLight<C, L, N>),
    Matrix(Matrix<C, L, N>),
    Voronoi(Voronoi<C, L, N>),
    StarryNight(StarryNight<C, L, N>),
    Solid(Solid<C, L, N>),
}

impl<const C: usize, const L: usize, const N: usize> WorldEnum<C, L, N> {
    pub fn as_tick<B: Buffer<RGB8, Point>>(
        &mut self,
    ) -> &mut dyn Tick<RGB8, Point, B, Ticker = Ticker> {
        match self {
            Self::Empty(w) => w,
            Self::Fire(w) => w,
            Self::NorthernLight(w) => w,
            Self::Matrix(w) => w,
            Self::Voronoi(w) => w,
            Self::StarryNight(w) => w,
            Self::Solid(w) => w,
        }
    }
}

impl<const C: usize, const L: usize, const N: usize> World for WorldEnum<C, L, N> {
    fn get_world(index: usize) -> WorldEnum<C, L, N> {
        match index {
            0 => WorldEnum::Empty(Empty::new()),
            1 => WorldEnum::Fire(Fire::new()),
            2 => WorldEnum::NorthernLight(NorthernLight::new()),
            3 => WorldEnum::Matrix(Matrix::new()),
            4 => WorldEnum::Voronoi(Voronoi::new()),
            5 => WorldEnum::StarryNight(StarryNight::new()),
            6 => WorldEnum::Solid(Solid::new()),
            _ => {
                defmt::panic!("World counter out of bounds")
            }
        }
    }
}
