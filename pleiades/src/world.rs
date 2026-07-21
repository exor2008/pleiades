use crate::{
    apds9960::Direction,
    buffer::{Buffer, Point},
    world::fire::Fire,
};
use embassy_time::Ticker;
use smart_leds::RGB8;

// pub mod empty;
pub mod fire;
// pub mod matrix;
// pub mod northen_light;
// pub mod solid;
// pub mod starry_night;
pub mod utils;
// pub mod voronoi;

const WORLDS: usize = 6;

pub trait Tick<Color, Coord, B>
where
    B: Buffer<Color, Coord>,
{
    type Ticker;

    fn tick(&mut self, buffer: &mut B);
    fn ticker(&mut self) -> &mut Self::Ticker;
    fn on_direction(&mut self, direction: Direction);
}

pub struct Switch {
    counter: usize,
    is_on: bool,
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

impl Switch {
    pub fn new() -> Self {
        Switch {
            counter: 1,
            is_on: true,
        }
    }

    pub fn switch_world<const C: usize, const L: usize>(&mut self) -> World<C, L> {
        self.counter += 1;
        self.counter = if self.counter > WORLDS {
            1
        } else {
            self.counter
        };
        Self::get_world(self.counter)
    }

    pub fn turn_off<const C: usize, const L: usize>(&mut self) -> World<C, L> {
        Self::get_world(0)
    }

    pub fn turn_on<const C: usize, const L: usize>(&mut self) -> World<C, L> {
        Self::get_world(self.counter)
    }

    pub fn switch_power<const C: usize, const L: usize>(&mut self) -> World<C, L> {
        match self.is_on {
            true => {
                self.is_on = false;
                self.turn_off::<C, L>()
            }
            false => {
                self.is_on = true;
                self.turn_on::<C, L>()
            }
        }
    }

    pub fn get_world<const C: usize, const L: usize>(index: usize) -> World<C, L> {
        match index {
            // 0 => World::empty_new(led),
            1 => World::Fire(Fire::new()),
            // 2 => World::northen_light_new(led),
            // 3 => World::matrix_new(led),
            // 4 => World::voronoi_new(led),
            // 5 => World::starry_night_new(led),
            // 6 => World::solid_new(led),
            _ => {
                defmt::panic!("World counter out of bounds")
            }
        }
    }
}

pub enum World<const C: usize, const L: usize> {
    Fire(Fire<C, L>),
}

impl<const C: usize, const L: usize> World<C, L> {
    pub fn as_tick<B: Buffer<RGB8, Point>>(
        &mut self,
    ) -> &mut dyn Tick<RGB8, Point, B, Ticker = Ticker> {
        match self {
            Self::Fire(w) => w,
        }
    }
}
