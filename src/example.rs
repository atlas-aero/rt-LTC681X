//! SPI bus mock for doc examples
use core::convert::Infallible;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

#[derive(Default)]
pub struct ExampleSPIBus {
    poll_count: usize,
    command: u8,
}

impl Transfer<u8> for ExampleSPIBus {
    type Error = Infallible;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        // Poll call
        if words.len() == 1 {
            self.poll_count += 1;

            return if self.poll_count >= 2 { Ok(&[0xff]) } else { Ok(&[0x0]) };
        }

        if words[1] == 0xff {
            return match self.command {
                // Status register A
                0b0001_0000 => Ok(&[0x12, 0x62, 0xA8, 0x62, 0x00, 0x7D, 0x31, 0x8A]),
                // Status register B
                0b0001_0010 => Ok(&[0x00, 0xC8, 0x00, 0x66, 0x00, 0x1B, 0xF1, 0x40]),
                // Cell voltage register B
                0b0000_0100 => Ok(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]),
                // Cell voltage register B
                0b0000_0110 => Ok(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]),
                // Cell voltage register C
                0b0000_1000 => Ok(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]),
                // Cell voltage register E
                0b0000_1001 => Ok(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]),
                // Aux voltage register A
                0b0000_1100 => Ok(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]),
                // Aux voltage register C
                0b0000_1101 => Ok(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]),
                _ => Ok(&[0x0; 8]),
            };
        }

        // Remember command for next read
        self.command = words[1];
        Ok(&[0x0])
    }
}

pub struct ExampleCSPin {}

impl OutputPin for ExampleCSPin {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
