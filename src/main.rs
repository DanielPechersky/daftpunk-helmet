#![no_std]
#![no_main]
#![feature(slice_as_chunks)]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, gpio, peripherals::Peripherals, prelude::*, spi, Delay, IO};
use max7219::MAX7219;

const ROWS: usize = 5;
const _: () = assert!(ROWS <= 8);
static ANIMATION: &[[u8; ROWS]] = include_bytes!("../out.bin").as_chunks::<ROWS>().0;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mosi = io.pins.gpio23;
    let sck = io.pins.gpio18;
    let cs = io.pins.gpio5;

    let spi = spi::master::Spi::new(peripherals.SPI3, 10u32.MHz(), spi::SpiMode::Mode0, &clocks)
        .with_pins(Some(sck), Some(mosi), gpio::NO_PIN, Some(cs));
    let mut max7219 = MAX7219::from_spi(1, spi).unwrap();

    max7219.set_intensity(0, 0xf).unwrap();
    max7219.power_on().unwrap();

    loop {
        for frame in ANIMATION {
            let mut display = [0; 8];
            display[..frame.len()].copy_from_slice(frame);
            max7219.write_raw(0, &display).unwrap();
            delay.delay_ms(1000u32 / 30);
        }
    }
}
