#![no_std]
#![no_main]

mod gpio;
mod usart;
mod spi;

use cortex_m_rt::entry;
use gpio::{Gpio, PinMode, PinValue};
use usart::Usart;
use spi::{Spi, SpiMode};

#[entry]
fn main() -> ! {
    let gpio = Gpio::new();
    let mut usart = usart::new_usart();
    let mut spi = spi::new_spi();

    gpio.configure_pin(5, PinMode::Output);

    // Configuration SPI
    spi.configure(1000000, SpiMode::Master);

    // Écriture d'un message dans le SPI
    spi.write(b'A');

    loop {
        if let Some(command) = usart.read() {
            match command {
                b'H' => gpio.write_pin(5, PinValue::High),
                b'L' => gpio.write_pin(5, PinValue::Low),
                _ => (),
            }
        }
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
