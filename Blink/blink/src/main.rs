#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

mod gpio;

use gpio::{Gpio, PinMode, PinValue};

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let gpio = Gpio::new();
    gpio.configure_pin(5, PinMode::Output);

    loop {
        gpio.write_pin(5, PinValue::High);
        for _ in 0..1_000_000 {}
        gpio.write_pin(5, PinValue::Low);
        for _ in 0..1_000_000 {}
    }
}
