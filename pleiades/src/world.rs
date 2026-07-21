use crate::{
    apds9960::Direction,
    buffer::{Buffer, Point, RGB8Buffer},
    color::Color,
    ws2812::LedWrite,
};
use pleiades_macro_derive::enum_world;
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
}

// pub trait Flush<Color, Led, const N: usize>
// where
//     Led: LedWrite<RGB8, N>,
// {
//     async fn flush(&mut self, led: &mut Led);
// }

pub trait OnDirection {
    fn on_direction(&mut self, direction: Direction);
}

// #[allow(clippy::large_enum_variant)]
// #[enum_world(Empty, Fire, NorthenLight, Matrix, Voronoi, StarryNight, Solid)]
// #[enum_world(Fire)]
// pub enum World<B, const C: usize, const L: usize, const N: usize, const N2: usize>
// where
//     B: Buffer<RGB8, Point>,
// {
//     // Empty(empty::Empty<'led, Led>),
//     Fire(fire::Fire<B, C, L>),
//     // NorthenLight(northen_light::NorthenLight<'led, Led, C, L, N>),
//     // Matrix(matrix::Matrix<'led, Led, C, L, N, N2>),
//     // Voronoi(voronoi::Voronoi<'led, Led, C, L, N>),
//     // StarryNight(starry_night::StarryNight<'led, Led, C, L, N>),
//     // Solid(solid::Solid<'led, Led, C, L, N>),
// }

// pub struct Switch {
//     counter: usize,
//     prev_counter: usize,
//     is_on: bool,
// }
//
// impl Default for Switch {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// impl Switch {
//     pub fn new() -> Self {
//         Switch {
//             counter: 1,
//             prev_counter: Default::default(),
//             is_on: true,
//         }
//     }
//
//     pub fn switch_world<
//         'led,
//         B: Buffer,
//         const C: usize,
//         const L: usize,
//         const N: usize,
//         const N2: usize,
//     >(
//         &mut self,
//         led: &'led mut B,
//     ) -> World<B, C, L, N, N2> {
//         // Destroy old world and return peripherial resources
//         self.counter += 1;
//         self.counter = if self.counter > WORLDS {
//             1
//         } else {
//             self.counter
//         };
//         self.get_world(led)
//     }
//
//     fn turn_off<
//         'led,
//         Led: Buffer,
//         const C: usize,
//         const L: usize,
//         const N: usize,
//         const N2: usize,
//     >(
//         &mut self,
//         led: &'led mut Led,
//     ) -> World<Led, C, L, N, N2> {
//         self.prev_counter = self.counter;
//         self.counter = 0;
//         self.get_world(led)
//     }
//
//     fn turn_on<
//         'led,
//         Led: Buffer,
//         const C: usize,
//         const L: usize,
//         const N: usize,
//         const N2: usize,
//     >(
//         &mut self,
//         led: &'led mut Led,
//     ) -> World<Led, C, L, N, N2> {
//         self.counter = self.prev_counter;
//         self.get_world(led)
//     }
//
//     pub fn switch_power<
//         'led,
//         Led: Buffer,
//         const C: usize,
//         const L: usize,
//         const N: usize,
//         const N2: usize,
//     >(
//         &mut self,
//         led: &'led mut Led,
//     ) -> World<Led, C, L, N, N2> {
//         match self.is_on {
//             true => {
//                 self.is_on = false;
//                 self.turn_off(led)
//             }
//             false => {
//                 self.is_on = true;
//                 self.turn_on(led)
//             }
//         }
//     }
//
//     fn get_world<
//         'led,
//         B: Buffer,
//         const C: usize,
//         const L: usize,
//         const N: usize,
//         const N2: usize,
//     >(
//         &mut self,
//         led: &'led mut B,
//     ) -> World<B, C, L, N, N2> {
//         match self.counter {
//             // 0 => World::empty_new(led),
//             1 => World::fire_new(),
//             // 2 => World::northen_light_new(led),
//             // 3 => World::matrix_new(led),
//             // 4 => World::voronoi_new(led),
//             // 5 => World::starry_night_new(led),
//             // 6 => World::solid_new(led),
//             _ => {
//                 defmt::panic!("World counter out of bounds")
//             }
//         }
//     }
// }
