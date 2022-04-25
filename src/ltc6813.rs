use crate::monitor::{NoPolling, ToCommandBitmap, ToFullCommand, LTC681X};
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

/// Cell voltage registers
#[derive(Copy, Clone, PartialEq)]
pub enum CellVoltageRegister {
    RegisterA,
    RegisterB,
    RegisterC,
    RegisterD,
    RegisterE,
    RegisterF,
}

/// Auxiliary registers
#[derive(Copy, Clone, PartialEq)]
pub enum AuxiliaryRegister {
    RegisterA,
    RegisterB,
    RegisterC,
    RegisterD,
}

impl<B, CS, const L: usize>
    LTC681X<B, CS, NoPolling, CellSelection, GPIOSelection, CellVoltageRegister, AuxiliaryRegister, L>
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

impl ToFullCommand for CellVoltageRegister {
    /// Returns the precalculated full command
    fn to_command(&self) -> [u8; 4] {
        match self {
            CellVoltageRegister::RegisterA => [0x00, 0x04, 0x07, 0xC2],
            CellVoltageRegister::RegisterB => [0x00, 0x06, 0x9A, 0x94],
            CellVoltageRegister::RegisterC => [0x00, 0x08, 0x5E, 0x52],
            CellVoltageRegister::RegisterD => [0x00, 0x0A, 0xC3, 0x04],
            CellVoltageRegister::RegisterE => [0x00, 0x09, 0xD5, 0x60],
            CellVoltageRegister::RegisterF => [0x00, 0x0B, 0x48, 0x36],
        }
    }
}

impl ToFullCommand for AuxiliaryRegister {
    /// Returns the precalculated full command
    fn to_command(&self) -> [u8; 4] {
        match self {
            AuxiliaryRegister::RegisterA => [0x00, 0xC, 0xEF, 0xCC],
            AuxiliaryRegister::RegisterB => [0x00, 0xE, 0x72, 0x9A],
            AuxiliaryRegister::RegisterC => [0x00, 0xD, 0x64, 0xFE],
            AuxiliaryRegister::RegisterD => [0x00, 0xF, 0xF9, 0xA8],
        }
    }
}
