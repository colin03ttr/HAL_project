pub mod atmega;
pub mod cortex;

#[derive(Clone, Copy)]
pub enum UsartMode {
    Transmit,
    Receive,
    TransmitAndReceive,
}

pub trait Usart {
    fn configure(&mut self, baudrate: u32, mode: UsartMode);
    fn write(&mut self, data: u8);
    fn read(&mut self) -> Option<u8>;
}

pub fn new_usart() -> impl Usart {
    #[cfg(feature = "atmega328p")]
    {
        atmega::AtmegaUsart::new()
    }

    #[cfg(feature = "cortex_m7")]
    {
        cortex::CortexUsart::new()
    }

    #[cfg(not(any(feature = "atmega328p", feature = "cortex_m7")))]
    {
        struct DummyUsart;
        impl Usart for DummyUsart {
            fn configure(&mut self, _baudrate: u32, _mode: UsartMode) {}
            fn write(&mut self, _data: u8) {}
            fn read(&mut self) -> Option<u8> { None }
        }

        DummyUsart
    }
}
