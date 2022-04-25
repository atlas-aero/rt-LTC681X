use crate::commands::{CMD_AUX_V_REG_A, CMD_AUX_V_REG_B, CMD_CELL_V_REG_A, CMD_CELL_V_REG_B};
use crate::monitor::{NoPolling, ToCommandBitmap, ToFullCommand, LTC681X};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Cell selection for ADC conversion
///
/// See page 63 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/LTC6810-1-6810-2.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq)]
pub enum CellSelection {
    /// All cells
    All = 0x0,
    Cell1 = 0x1,
    Cell2 = 0x2,
    Cell3 = 0x3,
    Cell4 = 0x4,
    Cell5 = 0x5,
    Cell6 = 0x6,
}

/// GPIO selection for ADC conversion,
///
/// See page 63 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/LTC6810-1-6810-2.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq)]
pub enum GPIOSelection {
    /// S0, GPIO 1-4 and 2nd Reference
    All = 0x0,
    S0 = 0x1,
    GPIO1 = 0x2,
    GPIO2 = 0x3,
    GPIO3 = 0x4,
    GPIO4 = 0x5,
    SecondReference = 0x6,
}

/// Cell voltage registers
#[derive(Copy, Clone, PartialEq)]
pub enum CellVoltageRegister {
    RegisterA,
    RegisterB,
}

/// Auxiliary registers
#[derive(Copy, Clone, PartialEq)]
pub enum AuxiliaryRegister {
    RegisterA,
    RegisterB,
}

impl<B, CS, const L: usize>
    LTC681X<B, CS, NoPolling, CellSelection, GPIOSelection, CellVoltageRegister, AuxiliaryRegister, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
{
    /// Creates a client instant for LTC6810 variant
    pub fn ltc6810(bus: B, cs: CS) -> Self {
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
            CellVoltageRegister::RegisterA => CMD_CELL_V_REG_A,
            CellVoltageRegister::RegisterB => CMD_CELL_V_REG_B,
        }
    }
}

impl ToFullCommand for AuxiliaryRegister {
    /// Returns the precalculated full command
    fn to_command(&self) -> [u8; 4] {
        match self {
            AuxiliaryRegister::RegisterA => CMD_AUX_V_REG_A,
            AuxiliaryRegister::RegisterB => CMD_AUX_V_REG_B,
        }
    }
}
