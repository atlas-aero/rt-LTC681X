use crate::monitor::{NoPolling, ToCommandBitmap, LTC681X};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Cell selection for ADC conversion
///
/// See page 62 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/ltc6813-1.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq)]
pub enum CellSelection {
    /// All cells
    All = 0x0,
    /// Cells 1, 7, 13
    Group1 = 0x1,
    /// Cells 2, 8, 14
    Group2 = 0x2,
    /// Cells 3, 9, 15
    Group3 = 0x3,
    /// Cells 4, 10, 16
    Group4 = 0x4,
    /// Cells 5, 11, 17
    Group5 = 0x5,
    /// cells 6, 12, 18
    Group6 = 0x6,
}

/// GPIO selection for ADC conversion,
///
/// See page 62 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/ltc6813-1.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq)]
pub enum GPIOSelection {
    /// GPIO 1-5, 2nd Reference, GPIO 6-9
    All = 0x0,
    /// GPIO 1 and GPIO 6
    Group1 = 0x1,
    /// GPIO 2 and GPIO 7
    Group2 = 0x2,
    /// GPIO 3 and GPIO 8
    Group3 = 0x3,
    /// GPIO 4 and GPIO 9
    Group4 = 0x4,
    /// GPIO 5
    Group5 = 0x5,
    /// 2nd Reference
    Group6 = 0x6,
}

impl<B, CS, const L: usize> LTC681X<B, CS, NoPolling, CellSelection, GPIOSelection, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
{
    pub fn ltc6813(bus: B, cs: CS) -> Self {
        LTC681X::new(bus, cs)
    }
}

impl ToCommandBitmap for CellSelection {
    fn to_bitmap(&self) -> u16 {
        *self as u16
    }
}

impl ToCommandBitmap for GPIOSelection {
    fn to_bitmap(&self) -> u16 {
        *self as u16
    }
}
