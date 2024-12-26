use crate::usart::{Usart, UsartMode};

pub struct AtmegaUsart {
    buffer: Option<u8>,
}

impl AtmegaUsart {
    pub fn new() -> Self {
        AtmegaUsart { buffer: None }
    }
}

impl Usart for AtmegaUsart {
    fn configure(&mut self, _baudrate: u32, _mode: UsartMode) {
    }

    fn write(&mut self, data: u8) {
        self.buffer = Some(data);
    }

    fn read(&mut self) -> Option<u8> {
        self.buffer.take()
    }
}
