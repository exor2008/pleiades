#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::dma::InterruptHandler as DmaInterruptHandler;
use embassy_rp::i2c::{self, Async, Config, InterruptHandler as I2CInterruptHandler};
use embassy_rp::peripherals::{DMA_CH0, I2C0, PIO0};
use embassy_rp::pio::{InterruptHandler as PioInterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Ticker};
use ledlab::apds9960::Apds9960;
use ledlab::utils::Command;
use ledlab::world::{OnDirection, Switch};
use ledlab::{CommandHandler, start};
use pleiades::buffer::{Point, RGB8Buffer};
use pleiades::world::WorldEnum;

// #[cfg(feature = "panic-probe")]
use panic_probe as _;
// #[cfg(feature = "panic-reset")]
// use panic_reset as _;

const WORLDS: usize = 6;
const NUM_LEDS_LINE: usize = 16;
const NUM_LEDS_COLUMN: usize = 16;
const NUM_LEDS: usize = NUM_LEDS_LINE * NUM_LEDS_COLUMN;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => I2CInterruptHandler<I2C0>;
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
    DMA_IRQ_0 => DmaInterruptHandler<DMA_CH0>;
});

static CHANNEL: Channel<ThreadModeRawMutex, Command, 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Start");

    // Init pins
    let p = embassy_rp::init(Default::default());
    let sda = p.PIN_20;
    let scl = p.PIN_21;

    // Init I2C and Apds9960 gesture sensor
    let i2c = i2c::I2c::new_async(p.I2C0, scl, sda, Irqs, Config::default());
    let apds = Apds9960::new(i2c);

    // Start sensor_task asynchronously
    spawner.spawn(sensor_task(apds).unwrap());

    // Init PIO to support WS2812 protocol
    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_22, &program);

    // Init 16x16 LED matrix buffer
    let mut buffer: RGB8Buffer<NUM_LEDS_LINE, NUM_LEDS> = RGB8Buffer::new();

    // Switcher to switch between the worlds
    let mut switch: Switch<WORLDS> = Switch::new();

    // Run main loop
    start(&mut buffer, &mut switch, &mut ws2812, Handler {}).await;
}

#[embassy_executor::task]
async fn sensor_task(mut apds: Apds9960<'static, I2C0, Async>) -> ! {
    apds.enable().await.unwrap();
    apds.powerup().await.unwrap();

    let mut ticker = Ticker::every(Duration::from_millis(10));

    loop {
        // if let Ok(d) = apds.read().await {
        //     defmt::info!("Dist: {}", d);
        // }
        apds.gesture().await;
        if let Some(command) = apds.command()
            && let Err(_err) = CHANNEL.try_send(command)
        {
            defmt::error!("Command channel buffer is full");
        }
        ticker.next().await;
    }
}

struct Handler;

impl
    CommandHandler<
        WorldEnum<NUM_LEDS_COLUMN, NUM_LEDS_LINE, NUM_LEDS>,
        Point,
        RGB8Buffer<NUM_LEDS_LINE, NUM_LEDS>,
        NUM_LEDS,
        WORLDS,
    > for Handler
{
    fn handle(
        &self,
        _buffer: &mut RGB8Buffer<NUM_LEDS_LINE, NUM_LEDS>,
        world: &mut WorldEnum<NUM_LEDS_COLUMN, NUM_LEDS_LINE, NUM_LEDS>,
        switch: &mut Switch<WORLDS>,
    ) {
        if let Ok(command) = CHANNEL.try_receive() {
            // defmt::info!("Command!: {}", command);
            match command {
                Command::Level(direction) => {
                    world.on_direction(direction);
                }
                Command::Swing => {
                    *world = switch
                        .switch_world::<WorldEnum<NUM_LEDS_COLUMN, NUM_LEDS_LINE, NUM_LEDS>>();
                }
                Command::SwitchPower => {
                    *world = switch
                        .switch_power::<WorldEnum<NUM_LEDS_COLUMN, NUM_LEDS_LINE, NUM_LEDS>>();
                }
            }
        }
    }
}
