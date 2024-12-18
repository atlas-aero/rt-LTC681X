//! SPI bus mock for doc examples
use core::convert::Infallible;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};

#[derive(Default)]
pub struct ExampleSPIDevice {
    poll_count: usize,
    command: u8,
}

impl ErrorType for ExampleSPIDevice {
    type Error = Infallible;
}

impl SpiDevice<u8> for ExampleSPIDevice {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        if operations.len() != 1 {
            panic!("Only one operation supported by example SPI client");
        }

        match &mut operations[0] {
            Operation::Read(buffer) => {
                // Poll call
                if buffer.len() == 1 {
                    self.poll_count += 1;
                    if self.poll_count >= 2 {
                        buffer[0] = 0xff
                    } else {
                        buffer[0] = 0x0
                    };
                }

                match self.command {
                    // Status register A
                    0b0001_0000 => buffer.copy_from_slice(&[0x12, 0x62, 0xA8, 0x62, 0x00, 0x7D, 0x31, 0x8A]),
                    // Status register B
                    0b0001_0010 => buffer.copy_from_slice(&[0x00, 0xC8, 0x00, 0x66, 0x00, 0x1B, 0xF1, 0x40]),
                    // Cell voltage register B
                    0b0000_0100 => buffer.copy_from_slice(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]),
                    // Cell voltage register B
                    0b0000_0110 => buffer.copy_from_slice(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]),
                    // Cell voltage register C
                    0b0000_1000 => buffer.copy_from_slice(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]),
                    // Cell voltage register E
                    0b0000_1001 => buffer.copy_from_slice(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]),
                    // Aux voltage register A
                    0b0000_1100 => buffer.copy_from_slice(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]),
                    // Aux voltage register C
                    0b0000_1101 => buffer.copy_from_slice(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]),
                    _ => buffer.copy_from_slice(&[0x0; 8]),
                };
            }
            Operation::Write(words) => {
                self.command = words[1];
            }
            Operation::Transfer(_, _) => panic!("Transfer operation not supported by example SPI client"),
            Operation::TransferInPlace(_) => panic!("TransferInPlace operation not supported by example SPI client"),
            Operation::DelayNs(_) => panic!("DelayNs operation not supported by example SPI client"),
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct ExampleSPIBus {
    poll_count: usize,
}

impl ErrorType for ExampleSPIBus {
    type Error = Infallible;
}

impl SpiBus<u8> for ExampleSPIBus {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        // Poll call
        if words.len() == 1 {
            self.poll_count += 1;
            if self.poll_count >= 2 {
                words[0] = 0xff
            } else {
                words[0] = 0x0
            };
        }

        Ok(())
    }

    fn write(&mut self, _words: &[u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn transfer(&mut self, _read: &mut [u8], _write: &[u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn transfer_in_place(&mut self, _words: &mut [u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

pub struct ExampleCSPin {}

impl embedded_hal::digital::ErrorType for ExampleCSPin {
    type Error = Infallible;
}

impl OutputPin for ExampleCSPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
