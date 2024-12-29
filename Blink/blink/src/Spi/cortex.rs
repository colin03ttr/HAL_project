use crate::spi::{Spi, SpiMode};

pub struct CortexSpi {
    buffer: Option<u8>,
}

impl CortexSpi {
    pub fn new() -> Self {
        CortexSpi { buffer: None }
    }
}

impl Spi for CortexSpi {
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
