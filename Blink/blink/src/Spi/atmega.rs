use crate::spi::{Spi, SpiMode};

pub struct AtmegaSpi {
    buffer: Option<u8>,
}

impl AtmegaSpi {
    pub fn new() -> Self {
        AtmegaSpi { buffer: None }
    }
}

impl Spi for AtmegaSpi {
    fn configure(&mut self, _baudrate: u32, _mode: SpiMode) {
        // Configuration simulée
    }

    fn write(&mut self, data: u8) {
        // Écrit dans le buffer simulé
        self.buffer = Some(data);
    }

    fn read(&mut self) -> Option<u8> {
        // Lit le buffer simulé
        self.buffer.take()
    }
}
