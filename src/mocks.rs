use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use mockall::mock;

#[derive(Debug, PartialEq)]
pub enum BusError {
    Error1,
}

mock! {
    pub SPIBus {}

    impl Transfer<u8> for SPIBus{
        type Error = BusError;

        fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'static [u8], BusError>;
    }
}

#[derive(Debug, PartialEq)]
pub enum PinError {
    Error1,
}

mock! {
    pub Pin {}

    impl OutputPin for Pin {
        type Error = PinError;

        fn set_low(&mut self) -> Result<(), PinError>;
        fn set_high(&mut self) -> Result<(), PinError>;
    }
}

pub struct BusMockBuilder {
    bus: MockSPIBus,
}

impl BusMockBuilder {
    pub fn new() -> Self {
        BusMockBuilder { bus: MockSPIBus::new() }
    }

    pub fn expect_command(mut self, cmd0: u8, cmd1: u8, pec0: u8, pec1: u8) -> Self {
        self.bus.expect_transfer().times(1).returning(move |data| {
            assert_eq!(4, data.len());
            assert_eq!(cmd0, data[0]);
            assert_eq!(cmd1, data[1]);
            assert_eq!(pec0, data[2]);
            assert_eq!(pec1, data[3]);

            Ok(&[0xff, 0xff, 0xff, 0xff])
        });

        self
    }

    pub fn expect_register_read(mut self, data: &'static [u8; 8]) -> Self {
        self.bus.expect_transfer().times(1).returning(move |command| {
            assert_eq!([0xff; 8], command);
            Ok(data)
        });

        self
    }

    pub fn into_mock(self) -> MockSPIBus {
        self.bus
    }
}
