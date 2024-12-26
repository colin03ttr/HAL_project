use crate::usart::{Usart, UsartMode};

pub struct CortexUsart {
    buffer: Option<u8>,
}

impl CortexUsart {
    pub fn new() -> Self {
        CortexUsart { buffer: None }
    }
}

impl Usart for CortexUsart {
    fn configure(&mut self, _baudrate: u32, _mode: UsartMode) {
    }

    fn write(&mut self, data: u8) {
        self.buffer = Some(data);
    }

    fn read(&mut self) -> Option<u8> {
        self.buffer.take()
    }
}
