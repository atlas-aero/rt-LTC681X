use crate::commands::{
    CMD_AUX_V_REG_A, CMD_AUX_V_REG_B, CMD_CELL_V_REG_A, CMD_CELL_V_REG_B, CMD_CELL_V_REG_C, CMD_CELL_V_REG_D,
};
use crate::monitor::{NoPolling, ToCommandBitmap, ToFullCommand, LTC681X};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Cell selection for ADC conversion
///
/// See page 61 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/LTC6811-1-6811-2.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq)]
pub enum CellSelection {
    /// All cells
    All = 0x0,
    /// Cells 1 and 7
    Pair1 = 0x1,
    /// Cells 2 and 8
    Pair2 = 0x2,
    /// Cells 3 and 9
    Pair3 = 0x3,
    /// Cells 4 and 10
    Pair4 = 0x4,
    /// Cells 5 and 11
    Pair5 = 0x5,
    /// Cells 6 and 12
    Pair6 = 0x6,
}

/// GPIO selection for ADC conversion,
///
/// See page 61 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/LTC6811-1-6811-2.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq)]
pub enum GPIOSelection {
    /// GPIO 1-5 and 2nd Reference
    All = 0x0,
    GPIO1 = 0x1,
    GPIO2 = 0x2,
    GPIO3 = 0x3,
    GPIO4 = 0x4,
    GPIO5 = 0x5,
    SecondReference = 0x6,
}

/// Cell voltage registers
#[derive(Copy, Clone, PartialEq)]
pub enum CellVoltageRegister {
    RegisterA,
    RegisterB,
    RegisterC,
    RegisterD,
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
    /// Creates a client instant for LTC6811 variant
    pub fn ltc6811(bus: B, cs: CS) -> Self {
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
            CellVoltageRegister::RegisterC => CMD_CELL_V_REG_C,
            CellVoltageRegister::RegisterD => CMD_CELL_V_REG_D,
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
