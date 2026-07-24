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
    world::{GetTicker, GetWorld, OnDirection, Tick},
};

pub mod empty;
pub mod fire;
pub mod matrix;
pub mod northern_light;
pub mod solid;
pub mod starry_night;
pub mod voronoi;

#[allow(clippy::large_enum_variant)]
pub enum WorldEnum<const C: usize, const L: usize, const N: usize> {
    Empty(Empty),
    Fire(Fire<C, L, N>),
    NorthernLight(NorthernLight<C, L, N>),
    Matrix(Matrix<C, L, N>),
    Voronoi(Voronoi<C, L, N>),
    StarryNight(StarryNight<C, L, N>),
    Solid(Solid<C, L, N>),
}

impl<const C: usize, const L: usize, const N: usize> GetTicker for WorldEnum<C, L, N> {
    fn get_ticker(&mut self) -> &mut Ticker {
        match self {
            Self::Empty(w) => w.get_ticker(),
            Self::Fire(w) => w.get_ticker(),
            Self::NorthernLight(w) => w.get_ticker(),
            Self::Matrix(w) => w.get_ticker(),
            Self::Voronoi(w) => w.get_ticker(),
            Self::StarryNight(w) => w.get_ticker(),
            Self::Solid(w) => w.get_ticker(),
        }
    }
}

impl<B, const C: usize, const L: usize, const N: usize> Tick<Point, B, N> for WorldEnum<C, L, N>
where
    B: Buffer<Point, N>,
{
    fn tick(&mut self, buffer: &mut B) {
        match self {
            Self::Empty(w) => w.tick(buffer),
            Self::Fire(w) => w.tick(buffer),
            Self::NorthernLight(w) => w.tick(buffer),
            Self::Matrix(w) => w.tick(buffer),
            Self::Voronoi(w) => w.tick(buffer),
            Self::StarryNight(w) => w.tick(buffer),
            Self::Solid(w) => w.tick(buffer),
        }
    }
}

impl<const C: usize, const L: usize, const N: usize> OnDirection for WorldEnum<C, L, N> {
    fn on_direction(&mut self, direction: ledlab::utils::Direction) {
        match self {
            Self::Empty(w) => w.on_direction(direction),
            Self::Fire(w) => w.on_direction(direction),
            Self::NorthernLight(w) => w.on_direction(direction),
            Self::Matrix(w) => w.on_direction(direction),
            Self::Voronoi(w) => w.on_direction(direction),
            Self::StarryNight(w) => w.on_direction(direction),
            Self::Solid(w) => w.on_direction(direction),
        }
    }
}

impl<const C: usize, const L: usize, const N: usize> GetWorld for WorldEnum<C, L, N> {
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
