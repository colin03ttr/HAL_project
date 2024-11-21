pub const DDRB: *mut u8 = 0x24 as *mut u8;
pub const PORTB: *mut u8 = 0x25 as *mut u8;
pub const PINB: *mut u8 = 0x23 as *mut u8;

pub struct Gpio {
    ddr: *mut u8,
    port: *mut u8,
    pin: *mut u8,
    max_pins: u8,
}

pub enum PinMode {
    Input,
    Output,
}

pub enum PinValue {
    High,
    Low,
}

impl Gpio {
    pub fn new() -> Self {
        Gpio {
            ddr: DDRB,
            port: PORTB,
            pin: PINB,
            max_pins: 8, // Atmega328p port B has 8 pins (0-7)
        }
    }

    pub fn configure_pin(&self, pin: u8, mode: PinMode) {
        if pin >= self.max_pins {
            panic!("Invalid pin number: {}", pin);
        }
        unsafe {
            match mode {
                PinMode::Input => core::ptr::write_volatile(
                    self.ddr,
                    core::ptr::read_volatile(self.ddr) & !(1 << pin),
                ),
                PinMode::Output => core::ptr::write_volatile(
                    self.ddr,
                    core::ptr::read_volatile(self.ddr) | (1 << pin),
                ),
            }
        }
    }

    pub fn write_pin(&self, pin: u8, value: PinValue) {
        if pin >= self.max_pins {
            panic!("Invalid pin number: {}", pin);
        }
        unsafe {
            match value {
                PinValue::High => core::ptr::write_volatile(
                    self.port,
                    core::ptr::read_volatile(self.port) | (1 << pin),
                ),
                PinValue::Low => core::ptr::write_volatile(
                    self.port,
                    core::ptr::read_volatile(self.port) & !(1 << pin),
                ),
            }
        }
    }

    pub fn read_pin(&self, pin: u8) -> PinValue {
        if pin >= self.max_pins {
            panic!("Invalid pin number: {}", pin);
        }
        unsafe {
            if core::ptr::read_volatile(self.pin) & (1 << pin) != 0 {
                PinValue::High
            } else {
                PinValue::Low
            }
        }
    }
}
