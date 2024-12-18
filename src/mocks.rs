use embedded_hal::digital::{ErrorType, OutputPin};
use embedded_hal::spi::{Error, ErrorKind, Operation, SpiBus, SpiDevice};
use mockall::mock;

#[derive(Debug, PartialEq, Eq)]
pub enum BusError {
    Error1,
}

mock! {
    pub SPIDevice {}

    impl embedded_hal::spi::ErrorType for SPIDevice { type Error = BusError; }

    impl SpiDevice<u8> for SPIDevice{
        fn transaction<'a>(&mut self, operations: &mut [Operation<'a, u8>]) -> Result<(), BusError>;
    }
}

mock! {
    pub SPIBus {}

    impl embedded_hal::spi::ErrorType for SPIBus { type Error = BusError; }

    impl SpiBus<u8> for SPIBus{
        fn read(&mut self, words: &mut [u8]) -> Result<(), BusError>;
        fn write(&mut self, words: &[u8]) -> Result<(), BusError>;
        fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), BusError>;
        fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), BusError>;
        fn flush(&mut self) -> Result<(), BusError>;
    }
}

impl Error for BusError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PinError {
    Error1,
}

mock! {
    pub Pin {}

    impl ErrorType for Pin { type Error = PinError; }

    impl OutputPin for Pin {
        fn set_low(&mut self) -> Result<(), PinError>;
        fn set_high(&mut self) -> Result<(), PinError>;
    }
}

impl embedded_hal::digital::Error for PinError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

pub struct DeviceMockBuilder {
    device: MockSPIDevice,
}

impl DeviceMockBuilder {
    pub fn new() -> Self {
        DeviceMockBuilder {
            device: MockSPIDevice::new(),
        }
    }

    pub fn expect_command(mut self, cmd0: u8, cmd1: u8, pec0: u8, pec1: u8) -> Self {
        self.device.expect_transaction().times(1).returning(move |operation| {
            assert_eq!(1, operation.len());

            match operation[0] {
                Operation::Write(cmd) => {
                    assert_eq!(4, cmd.len());
                    assert_eq!(cmd0, cmd[0]);
                    assert_eq!(cmd1, cmd[1]);
                    assert_eq!(pec0, cmd[2]);
                    assert_eq!(pec1, cmd[3]);
                }
                _ => panic!("Received unexpected operation type {:?}", operation[0]),
            }

            Ok(())
        });

        self
    }

    pub fn expect_register_read(mut self, data: &'static [u8; 8]) -> Self {
        self.device.expect_transaction().times(1).returning(move |operation| {
            assert_eq!(1, operation.len());

            match &mut operation[0] {
                Operation::Read(buffer) => {
                    buffer.copy_from_slice(data);
                }
                _ => panic!("Received unexpected operation type {:?}", operation[0]),
            }

            Ok(())
        });

        self
    }

    pub fn expect_register_write(mut self, expected: &'static [u8; 8]) -> Self {
        self.device.expect_transaction().times(1).returning(move |operation| {
            assert_eq!(1, operation.len());

            match operation[0] {
                Operation::Write(cmd) => assert_eq!(expected, cmd),
                _ => panic!("Received unexpected operation type {:?}", operation[0]),
            }

            Ok(())
        });

        self
    }

    pub fn into_mock(self) -> MockSPIDevice {
        self.device
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
        self.bus.expect_write().times(1).returning(move |data| {
            assert_eq!(4, data.len());
            assert_eq!(cmd0, data[0]);
            assert_eq!(cmd1, data[1]);
            assert_eq!(pec0, data[2]);
            assert_eq!(pec1, data[3]);

            Ok(())
        });

        self
    }

    pub fn expect_register_read(mut self, data: &'static [u8; 8]) -> Self {
        self.bus.expect_read().times(1).returning(move |buffer| {
            buffer.copy_from_slice(data);
            Ok(())
        });

        self
    }

    pub fn expect_register_write(mut self, expected: &'static [u8; 8]) -> Self {
        self.bus.expect_write().times(1).returning(move |data| {
            assert_eq!(expected, data);
            Ok(())
        });

        self
    }

    pub fn into_mock(self) -> MockSPIBus {
        self.bus
    }
}
