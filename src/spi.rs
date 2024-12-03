use core::fmt::{Debug, Formatter};
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorKind, ErrorType, Operation, SpiBus, SpiDevice};

/// When using this client the CS pin gets not automatically released after an operation for
/// enabling SDO line polling.
pub struct LatchingSpiDevice<B, CS> {
    /// SPI bus
    bus: B,

    /// SPI CS pin
    cs: CS,

    /// True if CS pin is still LOW. Avoids double action on is_adc_ready() calls
    cs_low: bool,
}

impl<B: SpiBus, CS: OutputPin> LatchingSpiDevice<B, CS> {
    pub fn new(bus: B, cs: CS) -> Self {
        Self { bus, cs, cs_low: false }
    }

    /// Releases the CS pine by setting it high again.
    /// Usually this is done when the devices operations is finished.
    pub(crate) fn release_cs(&mut self) -> Result<(), Error<B, CS>> {
        self.cs.set_high().map_err(Error::CSError)?;
        self.cs_low = false;

        Ok(())
    }
}

/// Possible SPI handling errors
pub enum Error<B: SpiBus, CS: OutputPin> {
    /// Error while transferring SPI data
    BusError(B::Error),

    /// Error while changing the CS pin output state
    CSError(CS::Error),
}

impl<B: SpiBus, CS: OutputPin> ErrorType for LatchingSpiDevice<B, CS> {
    type Error = Error<B, CS>;
}

impl<B: SpiBus, CS: OutputPin> SpiDevice for LatchingSpiDevice<B, CS> {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        if !self.cs_low {
            self.cs.set_low().map_err(Error::CSError)?;
            self.cs_low = true;
        }

        for operation in operations {
            let result = match operation {
                Operation::Read(buffer) => self.bus.read(buffer).map_err(Error::BusError),
                Operation::Write(buffer) => self.bus.write(buffer).map_err(Error::BusError),
                Operation::Transfer(rx, tx) => self.bus.transfer(rx, tx).map_err(Error::BusError),
                Operation::TransferInPlace(buffer) => self.bus.transfer_in_place(buffer).map_err(Error::BusError),
                Operation::DelayNs(_) => Ok(()),
            };

            if result.is_err() {
                let _ = self.cs.set_high();
                return result;
            }
        }

        Ok(())
    }
}

impl<B: SpiBus, CS: OutputPin> Debug for Error<B, CS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::BusError(_) => f.debug_struct("BusError").finish(),
            Error::CSError(_) => f.debug_struct("CsError").finish(),
        }
    }
}

impl<B: SpiBus, CS: OutputPin> embedded_hal::spi::Error for Error<B, CS> {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::BusError(e) => e.kind(),
            Error::CSError(_) => ErrorKind::ChipSelectFault,
        }
    }
}
