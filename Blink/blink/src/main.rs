#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

mod gpio;
mod usart;

use gpio::{Gpio, PinMode, PinValue};
use usart::Usart;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    // Initialize GPIO for testing (optional)
    let gpio = Gpio::new();
    gpio.configure_pin(5, PinMode::Output);

    // Initialize USART
    let usart = Usart::new();
    usart.init(9600); // Set baud rate to 9600

    // Transmit a test message
    usart.transmit_string("USART Initialized!\r\n");

    loop {
        // Blink an LED for visual confirmation
        gpio.write_pin(5, PinValue::High);
        for _ in 0..1_000_000 {}
        gpio.write_pin(5, PinValue::Low);
        for _ in 0..1_000_000 {}

        // Echo back received data
        let received = usart.receive();
        usart.transmit(received);
    }
}
