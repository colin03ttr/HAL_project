pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;
pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;
pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;
pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;
pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;
pub const UDR0: *mut u8 = 0xC6 as *mut u8;

pub struct Usart {
    ubrrh: *mut u8,
    ubrrl: *mut u8,
    ucsra: *mut u8,
    ucsrb: *mut u8,
    ucsrc: *mut u8,
    udr: *mut u8,
}

impl Usart {
    pub fn new() -> Self {
        Usart {
            ubrrh: UBRR0H,
            ubrrl: UBRR0L,
            ucsra: UCSR0A,
            ucsrb: UCSR0B,
            ucsrc: UCSR0C,
            udr: UDR0,
        }
    }

    /// Initialize USART with a given baud rate.
    pub fn init(&self, baud_rate: u32) {
        let ubrr_value: u16 = (16_000_000 / (16 * baud_rate) - 1) as u16; // Assuming 16 MHz clock

        unsafe {
            core::ptr::write_volatile(self.ubrrh, (ubrr_value >> 8) as u8); // High byte
            core::ptr::write_volatile(self.ubrrl, ubrr_value as u8);       // Low byte

            // Enable transmitter and receiver
            core::ptr::write_volatile(self.ucsrb, (1 << 3) | (1 << 4));

            // Set frame format: 8 data bits, 1 stop bit
            core::ptr::write_volatile(self.ucsrc, (1 << 1) | (1 << 2));
        }
    }

    /// Transmit a single byte
    pub fn transmit(&self, data: u8) {
        unsafe {
            // Wait for the transmit buffer to be empty
            while core::ptr::read_volatile(self.ucsra) & (1 << 5) == 0 {}
            core::ptr::write_volatile(self.udr, data);
        }
    }

    /// Receive a single byte
    pub fn receive(&self) -> u8 {
        unsafe {
            // Wait for data to be received
            while core::ptr::read_volatile(self.ucsra) & (1 << 7) == 0 {}
            core::ptr::read_volatile(self.udr)
        }
    }

    /// Transmit a string
    pub fn transmit_string(&self, data: &str) {
        for byte in data.bytes() {
            self.transmit(byte);
        }
    }
}
