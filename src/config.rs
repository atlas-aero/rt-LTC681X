use core::fmt::{Display, Formatter};

/// Abstracted configuration of configuration register(s)
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Computed value of register A
    pub(crate) register_a: [u8; 6],

    /// Computed value of register B,
    pub(crate) register_b: [u8; 6],
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            register_a: [
                0b1111_1000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ],
            register_b: [
                0b0000_1111,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ],
        }
    }
}

/// GPIO pins of LTC681X device.
/// Depending on the device type, not all pins may be available.
/// Configuring a pin that is not physically available has no effect.
#[derive(Copy, Clone, Debug)]
pub enum GPIO {
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
    GPIO5,
    GPIO6,
    GPIO7,
    GPIO8,
    GPIO9,
}

/// Cell indexes of the LTC681X device.
/// Depending on the device type, not all cells may be available.
/// Configuring a cell that is not physically available has no effect.
#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Cell1,
    Cell2,
    Cell3,
    Cell4,
    Cell5,
    Cell6,
    Cell7,
    Cell8,
    Cell9,
    Cell10,
    Cell11,
    Cell12,
    Cell13,
    Cell14,
    Cell15,
    Cell16,
    Cell17,
    Cell18,
}

/// Timeout duration for discharge timer
#[derive(Copy, Clone, Debug)]
pub enum DischargeTimeout {
    Disabled = 0x0,
    HalfMinute = 0x1,
    OneMinute = 0x2,
    TwoMinutes = 0x3,
    ThreeMinutes = 0x4,
    FourMinutes = 0x5,
    FiveMinutes = 0x6,
    TenMinutes = 0x7,
    FifteenMinutes = 0x8,
    TwentyMinutes = 0x9,
    ThirtyMinutes = 0xA,
    FortyMinutes = 0xB,
    SixtyMinutes = 0xC,
    SeventyFiveMinutes = 0xD,
    NinetyMinutes = 0xE,
    TwoHours = 0xF,
}

/// Digital Redundancy Path Selection
#[derive(Copy, Clone, Debug)]
pub enum DigitalRedundancyPath {
    /// Redundancy is applied sequentially to ADC1, ADC2 and ADC3 digital paths during cell conversions
    /// and applied to ADC1 during AUX and STATUS conversions
    All = 0x0,
    /// Redundancy is applied only to ADC1 digital path
    ADC1 = 0x1,
    /// Redundancy is applied only to ADC2 digital path
    ADC2 = 0x2,
    /// Redundancy is applied only to ADC3 digital path
    ADC3 = 0x3,
}

/// Given voltage is out-of-range for fitting in 12 bit integer
#[derive(Debug)]
pub struct VoltageOutOfRangeError {}

impl Display for VoltageOutOfRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Voltage overflows 12-bit integer")
    }
}

impl Configuration {
    /// Enables pull-down of the given GPIO pin
    pub fn enable_gpio_pull_down(&mut self, pin: GPIO) {
        match pin {
            GPIO::GPIO1 => self.register_a[0] &= 0b1111_0111,
            GPIO::GPIO2 => self.register_a[0] &= 0b1110_1111,
            GPIO::GPIO3 => self.register_a[0] &= 0b1101_1111,
            GPIO::GPIO4 => self.register_a[0] &= 0b1011_1111,
            GPIO::GPIO5 => self.register_a[0] &= 0b0111_1111,
            GPIO::GPIO6 => self.register_b[0] &= 0b1111_1110,
            GPIO::GPIO7 => self.register_b[0] &= 0b1111_1101,
            GPIO::GPIO8 => self.register_b[0] &= 0b1111_1011,
            GPIO::GPIO9 => self.register_b[0] &= 0b1111_0111,
        }
    }

    /// Enables pull-down of the given GPIO pin
    pub fn disable_gpio_pull_down(&mut self, pin: GPIO) {
        match pin {
            GPIO::GPIO1 => self.register_a[0] |= 0b0000_1000,
            GPIO::GPIO2 => self.register_a[0] |= 0b0001_0000,
            GPIO::GPIO3 => self.register_a[0] |= 0b0010_0000,
            GPIO::GPIO4 => self.register_a[0] |= 0b0100_0000,
            GPIO::GPIO5 => self.register_a[0] |= 0b1000_0000,
            GPIO::GPIO6 => self.register_b[0] |= 0b0000_0001,
            GPIO::GPIO7 => self.register_b[0] |= 0b0000_0010,
            GPIO::GPIO8 => self.register_b[0] |= 0b0000_0100,
            GPIO::GPIO9 => self.register_b[0] |= 0b0000_1000,
        }
    }

    /// References Remain Powered Up Until Watchdog Timeout
    pub fn enable_reference_power(&mut self) {
        self.register_a[0] |= 0b0000_0100
    }

    /// References Shut Down After Conversions (Default)
    pub fn disable_reference_power(&mut self) {
        self.register_a[0] &= 0b1111_1011
    }

    /// Enables the Discharge Timer for Discharge Switches
    pub fn enable_discharge_timer(&mut self) {
        self.register_a[0] |= 0b0000_0010
    }

    /// Disables Discharge Timer
    pub fn disable_discharge_timer(&mut self) {
        self.register_a[0] &= 0b1111_1101
    }

    /// Sets the under-voltage comparison voltage in uV
    pub fn set_uv_comp_voltage(&mut self, voltage: u32) -> Result<(), VoltageOutOfRangeError> {
        if voltage == 0 {
            self.register_a[1] = 0x0;
            self.register_a[2] &= 0b1111_0000;
            return Ok(());
        }

        if !(3200..=6553600).contains(&voltage) {
            return Err(VoltageOutOfRangeError {});
        }

        let value = ((voltage / 1600) - 1) as u16;

        self.register_a[1] = value as u8;
        self.register_a[2] &= 0b1111_0000;
        self.register_a[2] |= (value >> 8) as u8;

        Ok(())
    }

    /// Sets the over-voltage comparison voltage in uV
    pub fn set_ov_comp_voltage(&mut self, voltage: u32) -> Result<(), VoltageOutOfRangeError> {
        if voltage == 0 {
            self.register_a[2] &= 0b0000_1111;
            self.register_a[3] = 0x0;
            return Ok(());
        }

        if !(1600..=6552000).contains(&voltage) {
            return Err(VoltageOutOfRangeError {});
        }

        let value = (voltage / 1600) as u16;

        self.register_a[3] = (value >> 4) as u8;
        self.register_a[2] &= 0b0000_1111;
        self.register_a[2] |= (value << 4) as u8;

        Ok(())
    }

    /// Turn ON Shorting Switch for Cell x
    pub fn discharge_cell(&mut self, cell: Cell) {
        match cell {
            Cell::Cell1 => self.register_a[4] |= 0b0000_0001,
            Cell::Cell2 => self.register_a[4] |= 0b0000_0010,
            Cell::Cell3 => self.register_a[4] |= 0b0000_0100,
            Cell::Cell4 => self.register_a[4] |= 0b0000_1000,
            Cell::Cell5 => self.register_a[4] |= 0b0001_0000,
            Cell::Cell6 => self.register_a[4] |= 0b0010_0000,
            Cell::Cell7 => self.register_a[4] |= 0b0100_0000,
            Cell::Cell8 => self.register_a[4] |= 0b1000_0000,
            Cell::Cell9 => self.register_a[5] |= 0b0000_0001,
            Cell::Cell10 => self.register_a[5] |= 0b0000_0010,
            Cell::Cell11 => self.register_a[5] |= 0b0000_0100,
            Cell::Cell12 => self.register_a[5] |= 0b0000_1000,
            Cell::Cell13 => self.register_b[0] |= 0b0001_0000,
            Cell::Cell14 => self.register_b[0] |= 0b0010_0000,
            Cell::Cell15 => self.register_b[0] |= 0b0100_0000,
            Cell::Cell16 => self.register_b[0] |= 0b1000_0000,
            Cell::Cell17 => self.register_b[1] |= 0b0000_0001,
            Cell::Cell18 => self.register_b[1] |= 0b0000_0010,
        }
    }

    /// Sets the discharge timeout
    pub fn set_discharge_timeout(&mut self, timeout: DischargeTimeout) {
        self.register_a[5] &= 0b0000_1111;
        self.register_a[5] |= (timeout as u8) << 4;
    }

    /// Alternative ADC modes 14kHz, 3kHz, 1kHz or 2kHz
    pub fn set_alternative_adc_modes(&mut self) {
        self.register_a[0] |= 0b0000_0001
    }

    /// Default ADC modes 27kHz, 7kHz, 422Hz or 26Hz
    pub fn set_default_adc_modes(&mut self) {
        self.register_a[0] &= 0b1111_1110
    }

    /// Forces the digital redundancy comparison for ADC Conversions to fail
    pub fn force_digital_redundancy_fail(&mut self) {
        self.register_b[1] |= 0b0100_0000;
    }

    /// Sets the digital redundancy path
    pub fn set_digital_redundancy_path(&mut self, selection: DigitalRedundancyPath) {
        self.register_b[1] &= 0b1100_1111;
        self.register_b[1] |= (selection as u8) << 4;
    }

    /// Enables the discharge timer monitor function if the DTEN Pin is Asserted
    /// Otherwise (default) the discharge dimer monitor function is disabled. The normal discharge
    /// timer function will be enabled if the DTEN pin is asserted
    pub fn enable_discharge_monitor(&mut self) {
        self.register_b[1] |= 0b0000_1000;
    }
}
