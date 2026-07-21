use crate::apds9960::Direction;
use crate::buffer::{Buffer, Point};
use crate::color::ColorGradient;
use crate::perlin;
use crate::world::Tick;
use crate::world::utils::CooldownValue;
use core::cmp::max;
use embassy_rp::clocks::RoscRng;
use embassy_time::{Duration, Ticker};
use heapless::Vec;
use rand::Rng;
use smart_leds::RGB8;
use smart_leds::hsv::Hsv;

const HEIGHT_COOLDOWN: u8 = 1;
const HEIGHT_MIN: usize = 3;
const HEIGHT_MAX: usize = 15;
const HEIGHT_INIT: usize = 9;
const COLORS: usize = 4;
const MAX_SPARKS: usize = 2;
const SPAWN_COOLDOWN: usize = 60;

pub struct Fire<const C: usize, const L: usize> {
    noise: perlin::PerlinNoise,
    colormap: ColorGradient<COLORS>,
    height: CooldownValue<HEIGHT_COOLDOWN, HEIGHT_MIN, HEIGHT_MAX>,
    sparks: Vec<Spark, MAX_SPARKS>,
    ticker: Ticker,
    spawn_counter: usize,
    t: usize,
}

impl<const C: usize, const L: usize> Fire<C, L> {
    pub fn new() -> Self {
        let noise = perlin::PerlinNoise::new();
        let colormap = Fire::<C, L>::get_colormap();
        let height = CooldownValue::new(HEIGHT_INIT);
        let ticker = Ticker::every(Duration::from_millis(35));
        let sparks: Vec<Spark, MAX_SPARKS> = Vec::new();
        let spawn_counter = Default::default();

        Self {
            noise,
            colormap,
            height,
            sparks,
            ticker,
            spawn_counter,
            t: 0,
        }
    }
}

impl<const C: usize, const L: usize> Default for Fire<C, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B, const C: usize, const L: usize> Tick<RGB8, Point, B> for Fire<C, L>
where
    B: Buffer<RGB8, Point>,
{
    type Ticker = Ticker;

    fn tick(&mut self, buffer: &mut B) {
        buffer.clear();

        for x in 0..C {
            // Generate noise for fire shape
            let xx = x as f32 / 2.6;
            let yy = self.t as f32 / 10.0;
            let noise = self.noise.get2d([xx, yy]);
            let noise = (noise - 0.3) / 0.25; // [0..1]
            let noise = noise.clamp(0.0, 1.0);

            //Determine the height of fire pillar
            let height = (noise * (L - self.height.value()) as f32) as usize;
            let height = max(2, height);

            // Process the sparks
            self.spawn_spark(x, height);

            // Color every fire pillar pixel
            // and write it to buffer
            for y in L - height..L {
                let temp = (L - y - 1) as f32 / (height - 1) as f32;
                let color = self.colormap.get_noised(temp, -0.2, 0.2);
                buffer.write(Point { x, y }, color);
            }
        }
        self.process_sparks();
        self.draw_sparks(buffer);

        self.t = self.t.wrapping_add(1);
        // self.ticker.next().await;
    }

    fn ticker(&mut self) -> &mut Self::Ticker {
        &mut self.ticker
    }

    // impl<const C: usize, const L: usize> OnDirection for Fire<C, L> {
    fn on_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.colormap.change_value(20);
                self.height.down();
            }
            Direction::Down => {
                self.colormap.change_value(-20);
                self.height.up();
            }
        }
    }
}

// }

impl<const C: usize, const L: usize> Fire<C, L> {
    fn spawn_spark(&mut self, x: usize, height: usize) {
        self.spawn_counter += 1;
        if height < (C - 1)
            && perlin::fair_rand_float() > 0.857
            && self.spawn_counter >= SPAWN_COOLDOWN
        {
            let spark = Spark {
                x: x as isize,
                y: (C - 1 - height) as isize,
            };
            self.spawn_counter = 0;
            // Do not spawn spark if it's already too many sparks
            if self.sparks.push(spark).is_err() {}
        }
    }

    fn process_sparks(&mut self) {
        self.sparks.iter_mut().for_each(|spark| spark.up());
        self.sparks
            .retain(|spark| (spark.x >= 0) && (spark.x < C as isize) && (spark.y >= 0));
    }

    fn draw_sparks<B: Buffer<RGB8, Point>>(&mut self, buffer: &mut B) {
        let mut rng = RoscRng;
        let temp = rng.gen_range(0.8f32..=1.0);

        for spark in self.sparks.iter() {
            let color = self.colormap.get_noised(temp, 0.0, 0.2);
            let p = Point {
                x: spark.x as usize,
                y: spark.y as usize,
            };
            buffer.write(p, color);
        }
    }

    fn get_colormap() -> ColorGradient<COLORS> {
        let pos = [0.0, 0.2, 0.8, 1.01];
        let hsv = [
            Hsv {
                hue: 1,
                sat: 255,
                val: 48,
            },
            Hsv {
                hue: 6,
                sat: 255,
                val: 100,
            },
            Hsv {
                hue: 8,
                sat: 255,
                val: 150,
            },
            Hsv {
                hue: 10,
                sat: 255,
                val: 200,
            },
        ];
        ColorGradient::from_hsv(pos, hsv)
    }
}

#[derive(Debug)]
struct Spark {
    x: isize,
    y: isize,
}

impl Spark {
    fn up(&mut self) {
        let rnd = perlin::fair_rand_float();
        let dir = match rnd {
            rnd if rnd <= 0.2 => -1,
            rnd if rnd >= 0.6 => 1,
            _ => 0,
        };

        self.y -= 1;
        self.x += dir;
    }
}
