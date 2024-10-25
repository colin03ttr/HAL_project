#![no_std]
#![no_main]

use core::panic::PanicInfo;
const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *mut u8 = 0x23 as *mut u8;


#[panic_handler]
fn panic(_panic: &PanicInfo)->! {
   loop{}
}

use cortex_m_rt::entry;

struct Gpio {
    ddr: *mut u8,
    port: *mut u8,
    pin: *mut u8,
}

impl Gpio {
    fn new(ddr: *mut u8, port: *mut u8, pin: *mut u8) -> Self {
        Gpio { ddr, port, pin }
    }

    fn configure_pin(&self, pin: u8, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Input => core::ptr::write_volatile(self.ddr, core::ptr::read_volatile(self.ddr) & !(1 << pin)),
                PinMode::Output => core::ptr::write_volatile(self.ddr, core::ptr::read_volatile(self.ddr) | (1 << pin)),
            }
        }
    }

    fn write_pin(&self, pin: u8, value: PinValue) {
        unsafe {
            match value {
                PinValue::High => core::ptr::write_volatile(self.port, core::ptr::read_volatile(self.port) | (1 << pin)),
                PinValue::Low => core::ptr::write_volatile(self.port, core::ptr::read_volatile(self.port) & !(1 << pin)),
            }
        }
    }

    fn read_pin(&self, pin: u8) -> PinValue {
        unsafe {
            if core::ptr::read_volatile(self.pin) & (1 << pin) != 0 {
                PinValue::High
            } else {
                PinValue::Low
            }
        }
    }
}

enum PinMode {
    Input,
    Output,
}

enum PinValue {
    High,
    Low,
}

#[entry]
fn main() -> ! {
    let gpio = Gpio::new(DDRB, PORTB, PINB);
    // pin 5 as output
    gpio.configure_pin(5, PinMode::Output);
    loop { // loop to blink on gpio 5
        gpio.write_pin(5, PinValue::High);
        for _ in 0..1_000_000 {}
        gpio.write_pin(5, PinValue::Low);
        for _ in 0..1_000_000 {}
    }
    // loop to blink built-in led
    /* unsafe {
        core::ptr::write_volatile(DDRB, 0b00100000);
        loop{
            for _ in 0..1_000_000 {
                core::ptr::write_volatile(PORTB, 0b00100000);
            }
            for _ in 0..100_000 {
                core::ptr::write_volatile(PORTB, 0b00000000);
            }
        }
    } */
}