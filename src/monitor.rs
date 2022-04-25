use crate::monitor::Error::TransferError;
use crate::pec15::PEC15;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Poll Strategy
pub trait PollMethod<CS: OutputPin> {
    /// Handles the CS pin state after command has been sent
    fn end_command(&self, cs: &mut CS) -> Result<(), CS::Error>;
}

/// Leaves CS Low and waits until SDO goes high
pub struct SDOLinePolling {}

impl<CS: OutputPin> PollMethod<CS> for SDOLinePolling {
    fn end_command(&self, _cs: &mut CS) -> Result<(), CS::Error> {
        Ok(())
    }
}

/// No ADC polling is used
pub struct NoPolling {}

impl<CS: OutputPin> PollMethod<CS> for NoPolling {
    fn end_command(&self, cs: &mut CS) -> Result<(), CS::Error> {
        cs.set_high()
    }
}

/// ADC frequency and filtering settings
#[derive(Copy, Clone, PartialEq)]
pub enum ADCMode {
    /// 27kHz or 14kHz in case of CFGAR0=1 configuration
    Fast = 0x1,
    /// 7kHz or 3kHz in case of CFGAR0=1 configuration
    Normal = 0x2,
    /// 26Hz or 2kHz in case of CFGAR0=1 configuration
    Filtered = 0x3,
    /// 422Hz or 1kHz in case of CFGAR0=1 configuration
    Other = 0x0,
}

/// Error enum of LTC681X
#[derive(PartialEq)]
pub enum Error<B: Transfer<u8>, CS: OutputPin> {
    /// SPI transfer error
    TransferError(B::Error),

    /// Error while changing state of CS pin
    CSPinError(CS::Error),

    /// PEC checksum of returned data was invalid
    ChecksumMismatch,
}

/// Trait for casting command options to command bitmaps
pub trait ToCommandBitmap {
    /// Returns the command bitmap for the given argument.
    fn to_bitmap(&self) -> u16;
}

/// Trait for casting to constant (precomputed) commands
pub trait ToFullCommand {
    /// Returns the full command + PEC15
    fn to_command(&self) -> [u8; 4];
}

/// Public LTC681X client interface
///
/// L: Number of LTC681X devices in daisy chain
pub trait LTC681XClient<const L: usize> {
    type Error;

    /// Argument for the identification of cell groups, which depends on the exact device type.
    type CellSelection: ToCommandBitmap;

    /// Argument for the identification of GPIO groups, which depends on the exact device type.
    type GPIOSelection: ToCommandBitmap;

    /// Argument for cell voltage register selection. The available registers depend on the device.
    type CellVoltageRegister: ToFullCommand;

    /// Argument for aux register selection. The available registers depend on the device.
    type AuxiliaryRegister: ToFullCommand;

    /// Starts ADC conversion of cell voltages
    ///
    /// # Arguments
    ///
    /// * `mode`: ADC mode
    /// * `cells`: Measures the given cell group
    /// * `dcp`: True if discharge is permitted during conversion
    fn start_conv_cells(&mut self, mode: ADCMode, cells: Self::CellSelection, dcp: bool) -> Result<(), Self::Error>;

    /// Starts GPIOs ADC conversion
    ///
    /// # Arguments
    ///
    /// * `mode`: ADC mode
    /// * `channels`: Measures t:he given GPIO group
    fn start_conv_gpio(&mut self, mode: ADCMode, cells: Self::GPIOSelection) -> Result<(), Self::Error>;

    /// Reads and returns the cell voltages registers of the given register
    /// Returns one array for each device in daisy chain
    fn read_cell_voltages(&mut self, register: Self::CellVoltageRegister) -> Result<[[u16; 3]; L], Self::Error>;

    /// Reads the auxiliary voltages of the given register
    /// Returns one array for each device in daisy chain
    fn read_aux_voltages(&mut self, register: Self::AuxiliaryRegister) -> Result<[[u16; 3]; L], Self::Error>;
}

/// Public LTC681X interface for polling ADC status
pub trait PollClient {
    type Error;

    /// Returns true if the ADC is not busy
    fn adc_ready(&mut self) -> Result<bool, Self::Error>;
}

/// Client for LTC681X IC
pub struct LTC681X<B, CS, P, T1, T2, T3, T4, const L: usize>
where
    B: Transfer<u8>,
    CS: OutputPin,
    P: PollMethod<CS>,
    T1: ToCommandBitmap,
    T2: ToCommandBitmap,
    T3: ToFullCommand,
    T4: ToFullCommand,
{
    /// SPI bus
    bus: B,

    /// SPI CS pin
    cs: CS,

    /// Poll method used for type state
    poll_method: P,

    cell_selection_type: PhantomData<T1>,
    gpio_selection_type: PhantomData<T2>,
    cell_voltage_register_type: PhantomData<T3>,
    aux_register_type: PhantomData<T4>,
}

impl<B, CS, T1, T2, T3, T4, const L: usize> LTC681X<B, CS, NoPolling, T1, T2, T3, T4, L>
where
    B: Transfer<u8>,
    CS: OutputPin,

    T1: ToCommandBitmap,
    T2: ToCommandBitmap,
    T3: ToFullCommand,
    T4: ToFullCommand,
{
    pub(crate) fn new(bus: B, cs: CS) -> Self {
        LTC681X {
            bus,
            cs,
            poll_method: NoPolling {},
            cell_selection_type: PhantomData,
            gpio_selection_type: PhantomData,
            cell_voltage_register_type: PhantomData,
            aux_register_type: PhantomData,
        }
    }
}

impl<B, CS, P, T1, T2, T3, T4, const L: usize> LTC681XClient<L> for LTC681X<B, CS, P, T1, T2, T3, T4, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
    P: PollMethod<CS>,
    T1: ToCommandBitmap,
    T2: ToCommandBitmap,
    T3: ToFullCommand,
    T4: ToFullCommand,
{
    type Error = Error<B, CS>;
    type CellSelection = T1;
    type GPIOSelection = T2;
    type CellVoltageRegister = T3;
    type AuxiliaryRegister = T4;

    /// See [LTC681XClient::start_conv_cells](LTC681XClient#tymethod.start_conv_cells)
    fn start_conv_cells(&mut self, mode: ADCMode, cells: Self::CellSelection, dcp: bool) -> Result<(), Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        let mut command: u16 = 0b0000_0010_0110_0000;

        command |= (mode as u16) << 7;
        command |= cells.to_bitmap();

        if dcp {
            command |= 0b0001_0000;
        }

        self.send_command(command).map_err(Error::TransferError)?;
        self.poll_method.end_command(&mut self.cs).map_err(Error::CSPinError)
    }

    /// See [LTC681XClient::start_conv_gpio](LTC681XClient#tymethod.start_conv_gpio)
    fn start_conv_gpio(&mut self, mode: ADCMode, channels: Self::GPIOSelection) -> Result<(), Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        let mut command: u16 = 0b0000_0100_0110_0000;

        command |= (mode as u16) << 7;
        command |= channels.to_bitmap();

        self.send_command(command).map_err(Error::TransferError)?;
        self.poll_method.end_command(&mut self.cs).map_err(Error::CSPinError)
    }

    /// See [LTC681XClient::read_cell_voltages](LTC681XClient#tymethod.read_cell_voltages)
    fn read_cell_voltages(&mut self, register: Self::CellVoltageRegister) -> Result<[[u16; 3]; L], Error<B, CS>> {
        self.read_daisy_chain(register.to_command())
    }

    /// See [LTC681XClient::read_aux_voltages](LTC681XClient#tymethod.read_aux_voltages)
    fn read_aux_voltages(&mut self, register: Self::AuxiliaryRegister) -> Result<[[u16; 3]; L], Error<B, CS>> {
        self.read_daisy_chain(register.to_command())
    }
}

impl<B, CS, P, T1, T2, T3, T4, const L: usize> LTC681X<B, CS, P, T1, T2, T3, T4, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
    P: PollMethod<CS>,
    T1: ToCommandBitmap,
    T2: ToCommandBitmap,
    T3: ToFullCommand,
    T4: ToFullCommand,
{
    /// Sends the given command. Calculates and attaches the PEC checksum
    fn send_command(&mut self, command: u16) -> Result<(), B::Error> {
        let mut data = [(command >> 8) as u8, command as u8, 0x0, 0x0];
        let pec = PEC15::calc(&data[0..2]);

        data[2] = pec[0];
        data[3] = pec[1];

        self.bus.transfer(&mut data)?;
        Ok(())
    }

    /// Send the given read command and returns the response of all devices in daisy chain
    fn read_daisy_chain(&mut self, mut command: [u8; 4]) -> Result<[[u16; 3]; L], Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        self.bus.transfer(&mut command).map_err(Error::TransferError)?;

        let mut result = [[0, 0, 0]; L];
        for item in result.iter_mut().take(L) {
            *item = self.read()?;
        }

        self.cs.set_high().map_err(Error::CSPinError)?;
        Ok(result)
    }

    /// Reads a register
    fn read(&mut self) -> Result<[u16; 3], Error<B, CS>> {
        let mut command = [0xff_u8; 8];
        let result = self.bus.transfer(&mut command).map_err(TransferError)?;

        let pec = PEC15::calc(&result[0..6]);
        if pec[0] != result[6] || pec[1] != result[7] {
            return Err(Error::ChecksumMismatch);
        }

        let mut registers = [result[0] as u16, result[2] as u16, result[4] as u16];
        registers[0] |= (result[1] as u16) << 8;
        registers[1] |= (result[3] as u16) << 8;
        registers[2] |= (result[5] as u16) << 8;

        Ok(registers)
    }

    /// Enables SDO ADC polling
    ///
    /// After entering a conversion command, the SDO line is driven low when the device is busy
    /// performing conversions. SDO is pulled high when the device completes conversions.
    pub fn enable_sdo_polling(self) -> LTC681X<B, CS, SDOLinePolling, T1, T2, T3, T4, L> {
        LTC681X {
            bus: self.bus,
            cs: self.cs,
            poll_method: SDOLinePolling {},
            cell_selection_type: PhantomData,
            gpio_selection_type: PhantomData,
            cell_voltage_register_type: PhantomData,
            aux_register_type: PhantomData,
        }
    }
}

impl<B, CS, T1, T2, T3, T4, const L: usize> PollClient for LTC681X<B, CS, SDOLinePolling, T1, T2, T3, T4, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
    T1: ToCommandBitmap,
    T2: ToCommandBitmap,
    T3: ToFullCommand,
    T4: ToFullCommand,
{
    type Error = Error<B, CS>;

    /// Returns false if the ADC is busy
    /// If ADC is ready, CS line is pulled high
    fn adc_ready(&mut self) -> Result<bool, Self::Error> {
        let mut command = [0xff];
        let result = self.bus.transfer(&mut command).map_err(Error::TransferError)?;

        if result[0] == 0xff {
            self.cs.set_high().map_err(Error::CSPinError)?;
            return Ok(true);
        }

        Ok(false)
    }
}

impl<B: Transfer<u8>, CS: OutputPin> Debug for Error<B, CS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::TransferError(_) => f.debug_struct("TransferError").finish(),
            Error::CSPinError(_) => f.debug_struct("CSPinError").finish(),
            Error::ChecksumMismatch => f.debug_struct("ChecksumMismatch").finish(),
        }
    }
}
