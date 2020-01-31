#![no_main]
#![no_std]

extern crate panic_halt;

use lpc8xx_hal::{cortex_m_rt::entry, prelude::*};

#[entry]
fn main() -> ! {
    let p = lpc8xx_hal::Peripherals::take().unwrap();
    let swm = p.SWM.split();
    let mut syscon = p.SYSCON.split();
    let gpio = p.GPIO.enable(&mut syscon.handle);
    let led = swm.pins.pio1_1.into_gpio_pin(&gpio);
    let mut led = led.into_output();
    loop {
        for _ in 0..1_000_000 {
            led.set_high().unwrap();
        }
        for _ in 0..1_000_000 {
            led.set_low().unwrap();
        }
    }
}
