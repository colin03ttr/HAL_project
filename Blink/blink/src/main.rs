#![no_std]
#![no_main]

mod gpio;
mod usart;

use cortex_m_rt::entry;
use gpio::{Gpio, PinMode, PinValue};
use usart::{Usart};

#[entry]
fn main() -> ! {
    let gpio = Gpio::new();
    let mut usart = usart::new_usart();

    gpio.configure_pin(5, PinMode::Output);

    // Écriture d'un message dans le USART (simulation)
    usart.write(b'H');

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
