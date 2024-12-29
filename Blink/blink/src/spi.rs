pub mod atmega;
pub mod cortex;

pub trait Spi {
    fn configure(&mut self, baudrate: u32, mode: SpiMode);
    fn write(&mut self, data: u8);
    fn read(&mut self) -> Option<u8>;
}

#[derive(Clone, Copy)]
pub enum SpiMode {
    Master,
    Slave,
}

pub fn new_spi() -> impl Spi {
    #[cfg(feature = "atmega328p")]
    {
        atmega::AtmegaSpi::new()
    }

    #[cfg(feature = "cortex_m7")]
    {
        cortex::CortexSpi::new()
    }

    #[cfg(not(any(feature = "atmega328p", feature = "cortex_m7")))]
    {
        struct DummySpi;
        impl Spi for DummySpi {
            fn configure(&mut self, _baudrate: u32, _mode: SpiMode) {}
            fn write(&mut self, _data: u8) {}
            fn read(&mut self) -> Option<u8> { None }
        }

        DummySpi
    }
}
