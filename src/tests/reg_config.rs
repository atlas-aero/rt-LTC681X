use crate::config::{Cell, Configuration, DigitalRedundancyPath, DischargeTimeout, GPIO};

#[test]
fn test_enable_gpio_pull_down_gpio1() {
    let mut config = Configuration::default();

    config.enable_gpio_pull_down(GPIO::GPIO1);
    assert_eq!(0b1111_0000, config.register_a[0]);
    assert_default(0, &config);

    config = Configuration::default();
    config.enable_gpio_pull_down(GPIO::GPIO2);
    assert_eq!(0b1110_1000, config.register_a[0]);
    assert_default(0, &config);

    config = Configuration::default();
    config.enable_gpio_pull_down(GPIO::GPIO3);
    assert_eq!(0b1101_1000, config.register_a[0]);
    assert_default(0, &config);

    config = Configuration::default();
    config.enable_gpio_pull_down(GPIO::GPIO4);
    assert_eq!(0b1011_1000, config.register_a[0]);
    assert_default(0, &config);

    config = Configuration::default();
    config.enable_gpio_pull_down(GPIO::GPIO5);
    assert_eq!(0b0111_1000, config.register_a[0]);
    assert_default(0, &config);

    config = Configuration::default();
    config.enable_gpio_pull_down(GPIO::GPIO6);
    assert_eq!(0b0000_1110, config.register_b[0]);
    assert_default(6, &config);

    config = Configuration::default();
    config.enable_gpio_pull_down(GPIO::GPIO7);
    assert_eq!(0b0000_1101, config.register_b[0]);
    assert_default(6, &config);

    config = Configuration::default();
    config.enable_gpio_pull_down(GPIO::GPIO8);
    assert_eq!(0b0000_1011, config.register_b[0]);
    assert_default(6, &config);

    config = Configuration::default();
    config.enable_gpio_pull_down(GPIO::GPIO9);
    assert_eq!(0b0000_0111, config.register_b[0]);
    assert_default(6, &config);
}

#[test]
fn test_disable_gpio_pull_down_gpio1() {
    let mut config = Configuration::default();
    enable_all_gpio_pull_down_a(&mut config);
    config.disable_gpio_pull_down(GPIO::GPIO1);

    assert_eq!(0b0000_1000, config.register_a[0]);
    assert_default(0, &config);
}

#[test]
fn test_disable_gpio_pull_down_gpio2() {
    let mut config = Configuration::default();
    enable_all_gpio_pull_down_a(&mut config);
    config.disable_gpio_pull_down(GPIO::GPIO2);

    assert_eq!(0b0001_0000, config.register_a[0]);
    assert_default(0, &config);
}

#[test]
fn test_disable_gpio_pull_down_gpio3() {
    let mut config = Configuration::default();
    enable_all_gpio_pull_down_a(&mut config);
    config.disable_gpio_pull_down(GPIO::GPIO3);

    assert_eq!(0b0010_0000, config.register_a[0]);
    assert_default(0, &config);
}

#[test]
fn test_disable_gpio_pull_down_gpio4() {
    let mut config = Configuration::default();
    enable_all_gpio_pull_down_a(&mut config);
    config.disable_gpio_pull_down(GPIO::GPIO4);

    assert_eq!(0b0100_0000, config.register_a[0]);
    assert_default(0, &config);
}

#[test]
fn test_disable_gpio_pull_down_gpio5() {
    let mut config = Configuration::default();
    enable_all_gpio_pull_down_a(&mut config);
    config.disable_gpio_pull_down(GPIO::GPIO5);

    assert_eq!(0b1000_0000, config.register_a[0]);
    assert_default(0, &config);
}

#[test]
fn test_disable_gpio_pull_down_gpio6() {
    let mut config = Configuration::default();
    enable_all_gpio_pull_down_b(&mut config);
    config.disable_gpio_pull_down(GPIO::GPIO6);

    assert_eq!(0b0000_0001, config.register_b[0]);
    assert_default(6, &config);
}

#[test]
fn test_disable_gpio_pull_down_gpio7() {
    let mut config = Configuration::default();
    enable_all_gpio_pull_down_b(&mut config);
    config.disable_gpio_pull_down(GPIO::GPIO7);

    assert_eq!(0b0000_0010, config.register_b[0]);
    assert_default(6, &config);
}

#[test]
fn test_disable_gpio_pull_down_gpio8() {
    let mut config = Configuration::default();
    enable_all_gpio_pull_down_b(&mut config);
    config.disable_gpio_pull_down(GPIO::GPIO8);

    assert_eq!(0b0000_0100, config.register_b[0]);
    assert_default(6, &config);
}

#[test]
fn test_disable_gpio_pull_down_gpio9() {
    let mut config = Configuration::default();
    enable_all_gpio_pull_down_b(&mut config);
    config.disable_gpio_pull_down(GPIO::GPIO9);

    assert_eq!(0b0000_1000, config.register_b[0]);
    assert_default(6, &config);
}

#[test]
fn test_reference_power() {
    let mut config = Configuration::default();

    config.enable_reference_power();
    assert_eq!(0b1111_1100, config.register_a[0]);
    assert_default(0, &config);

    config.disable_reference_power();
    assert_eq!(0b1111_1000, config.register_a[0]);
    assert_default(0, &config);
}

#[test]
fn test_toggle_discharge_timer() {
    let mut config = Configuration::default();

    config.enable_discharge_timer();
    assert_eq!(0b1111_1010, config.register_a[0]);
    assert_default(0, &config);

    config.disable_discharge_timer();
    assert_eq!(0b1111_1000, config.register_a[0]);
    assert_default(0, &config);
}

#[test]
fn test_adc_modes() {
    let mut config = Configuration::default();

    config.set_alternative_adc_modes();
    assert_eq!(0b1111_1001, config.register_a[0]);
    assert_default(0, &config);

    config.set_default_adc_modes();
    assert_eq!(0b1111_1000, config.register_a[0]);
    assert_default(0, &config);
}

#[test]
fn test_set_uv_comp_voltage() {
    let mut config = Configuration::default();

    config.set_uv_comp_voltage(3_200_000).unwrap();
    assert_eq!(0b1100_1111, config.register_a[1]);
    assert_eq!(0b0000_0111, config.register_a[2]);

    config.set_uv_comp_voltage(0).unwrap();
    assert_eq!(0b0000_0000, config.register_a[1]);
    assert_eq!(0b0000_0000, config.register_a[2]);

    config.set_uv_comp_voltage(2_850_000).unwrap();
    assert_eq!(0b1111_0100, config.register_a[1]);
    assert_eq!(0b0000_0110, config.register_a[2]);

    config.set_uv_comp_voltage(4_100_000).unwrap();
    assert_eq!(0b0000_0001, config.register_a[1]);
    assert_eq!(0b0000_1010, config.register_a[2]);

    config.set_uv_comp_voltage(6_553_600).unwrap();
    assert_eq!(0b1111_1111, config.register_a[1]);
    assert_eq!(0b0000_1111, config.register_a[2]);

    config.set_uv_comp_voltage(3200).unwrap();
    assert_eq!(0b0000_0001, config.register_a[1]);
    assert_eq!(0b0000_0000, config.register_a[2]);

    config.set_uv_comp_voltage(3_200_000).unwrap();
    config.set_ov_comp_voltage(4_160_000).unwrap();
    assert_eq!(0b1100_1111, config.register_a[1]);
    assert_eq!(0b1000_0111, config.register_a[2]);
    assert_eq!(0b1010_0010, config.register_a[3]);
}

#[test]
fn test_set_uv_comp_voltage_out_of_range() {
    let mut config = Configuration::default();
    assert!(config.set_uv_comp_voltage(6_553_601).is_err());
    assert!(config.set_uv_comp_voltage(3199).is_err());
}

#[test]
fn test_set_ov_comp_voltage() {
    let mut config = Configuration::default();

    config.set_ov_comp_voltage(4_160_000).unwrap();
    assert_eq!(0b1010_0010, config.register_a[3]);
    assert_eq!(0b1000_0000, config.register_a[2]);

    config.set_ov_comp_voltage(0).unwrap();
    assert_eq!(0b0000_0000, config.register_a[3]);
    assert_eq!(0b0000_0000, config.register_a[2]);

    config.set_ov_comp_voltage(4_250_000).unwrap();
    assert_eq!(0b1010_0110, config.register_a[3]);
    assert_eq!(0b0000_0000, config.register_a[2]);

    config.set_ov_comp_voltage(1_000_000).unwrap();
    assert_eq!(0b0010_0111, config.register_a[3]);
    assert_eq!(0b0001_0000, config.register_a[2]);

    config.set_ov_comp_voltage(6_552_000).unwrap();
    assert_eq!(0b1111_1111, config.register_a[3]);
    assert_eq!(0b1111_0000, config.register_a[2]);

    config.set_ov_comp_voltage(1600).unwrap();
    assert_eq!(0b0000_0000, config.register_a[3]);
    assert_eq!(0b0001_0000, config.register_a[2]);

    config.set_ov_comp_voltage(4_160_000).unwrap();
    config.set_uv_comp_voltage(3_200_000).unwrap();
    assert_eq!(0b1100_1111, config.register_a[1]);
    assert_eq!(0b1000_0111, config.register_a[2]);
    assert_eq!(0b1010_0010, config.register_a[3]);
}

#[test]
fn test_set_ov_comp_voltage_out_of_range() {
    let mut config = Configuration::default();
    assert!(config.set_ov_comp_voltage(6_552_001).is_err());
    assert!(config.set_ov_comp_voltage(1599).is_err());
}

#[test]
fn test_discharge_cell() {
    let mut config = Configuration::default();

    config.discharge_cell(Cell::Cell1);
    assert_eq!(0b0000_0001, config.register_a[4]);
    assert_default(4, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell2);
    assert_eq!(0b0000_0010, config.register_a[4]);
    assert_default(4, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell3);
    assert_eq!(0b0000_0100, config.register_a[4]);
    assert_default(4, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell4);
    assert_eq!(0b0000_1000, config.register_a[4]);
    assert_default(4, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell5);
    assert_eq!(0b0001_0000, config.register_a[4]);
    assert_default(4, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell6);
    assert_eq!(0b0010_0000, config.register_a[4]);
    assert_default(4, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell7);
    assert_eq!(0b0100_0000, config.register_a[4]);
    assert_default(4, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell8);
    assert_eq!(0b1000_0000, config.register_a[4]);
    assert_default(4, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell9);
    assert_eq!(0b0000_0001, config.register_a[5]);
    assert_default(5, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell10);
    assert_eq!(0b0000_0010, config.register_a[5]);
    assert_default(5, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell11);
    assert_eq!(0b0000_0100, config.register_a[5]);
    assert_default(5, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell12);
    assert_eq!(0b0000_1000, config.register_a[5]);
    assert_default(5, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell13);
    assert_eq!(0b0001_1111, config.register_b[0]);
    assert_default(6, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell14);
    assert_eq!(0b0010_1111, config.register_b[0]);
    assert_default(6, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell15);
    assert_eq!(0b0100_1111, config.register_b[0]);
    assert_default(6, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell16);
    assert_eq!(0b1000_1111, config.register_b[0]);
    assert_default(6, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell17);
    assert_eq!(0b0000_0001, config.register_b[1]);
    assert_default(7, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell18);
    assert_eq!(0b0000_0010, config.register_b[1]);
    assert_default(7, &config);

    config = Configuration::default();
    config.discharge_cell(Cell::Cell1);
    config.discharge_cell(Cell::Cell5);
    config.discharge_cell(Cell::Cell7);
    config.discharge_cell(Cell::Cell10);
    config.discharge_cell(Cell::Cell11);
    config.discharge_cell(Cell::Cell14);
    config.discharge_cell(Cell::Cell16);
    assert_eq!(0b0101_0001, config.register_a[4]);
    assert_eq!(0b0000_0110, config.register_a[5]);
    assert_eq!(0b1010_1111, config.register_b[0]);
    assert_eq!(0b0000_0000, config.register_b[1]);
}

#[test]
fn test_set_discharge_timeout() {
    let mut config = Configuration::default();

    config.set_discharge_timeout(DischargeTimeout::Disabled);
    assert_eq!(0b0000_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::HalfMinute);
    assert_eq!(0b0001_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::OneMinute);
    assert_eq!(0b0010_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::TwoMinutes);
    assert_eq!(0b0011_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::ThreeMinutes);
    assert_eq!(0b0100_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::FourMinutes);
    assert_eq!(0b0101_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::FiveMinutes);
    assert_eq!(0b0110_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::TenMinutes);
    assert_eq!(0b0111_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::FifteenMinutes);
    assert_eq!(0b1000_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::TwentyMinutes);
    assert_eq!(0b1001_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::ThirtyMinutes);
    assert_eq!(0b1010_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::FortyMinutes);
    assert_eq!(0b1011_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::SixtyMinutes);
    assert_eq!(0b1100_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::SeventyFiveMinutes);
    assert_eq!(0b1101_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::NinetyMinutes);
    assert_eq!(0b1110_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::TwoHours);
    assert_eq!(0b1111_0000, config.register_a[5]);
    assert_default(5, &config);

    config.set_discharge_timeout(DischargeTimeout::SeventyFiveMinutes);
    config.discharge_cell(Cell::Cell10);
    assert_eq!(0b1101_0010, config.register_a[5]);
    assert_default(5, &config);
}

#[test]
fn test_force_digital_redundancy_fail() {
    let mut config = Configuration::default();

    config.force_digital_redundancy_fail();
    assert_eq!(0b0100_0000, config.register_b[1]);
    assert_default(7, &config);
}

#[test]
fn test_set_digital_redundancy_path() {
    let mut config = Configuration::default();

    config.set_digital_redundancy_path(DigitalRedundancyPath::All);
    assert_eq!(0b0000_0000, config.register_b[1]);
    assert_default(7, &config);

    config.set_digital_redundancy_path(DigitalRedundancyPath::ADC1);
    assert_eq!(0b0001_0000, config.register_b[1]);
    assert_default(7, &config);

    config.set_digital_redundancy_path(DigitalRedundancyPath::ADC2);
    assert_eq!(0b0010_0000, config.register_b[1]);
    assert_default(7, &config);

    config.set_digital_redundancy_path(DigitalRedundancyPath::ADC3);
    assert_eq!(0b0011_0000, config.register_b[1]);
    assert_default(7, &config);

    config.set_digital_redundancy_path(DigitalRedundancyPath::ADC2);
    config.discharge_cell(Cell::Cell17);
    assert_eq!(0b0010_0001, config.register_b[1]);
    assert_default(7, &config);
}

#[test]
fn test_enable_discharge_monitor() {
    let mut config = Configuration::default();

    config.enable_discharge_monitor();
    assert_eq!(0b0000_1000, config.register_b[1]);
    assert_default(7, &config);
}

#[test]
fn test_eq_false_register_a() {
    let mut a = Configuration::default();
    let b = Configuration::default();

    a.discharge_cell(Cell::Cell1);
    assert_ne!(a, b);
}

#[test]
fn test_eq_false_register_b() {
    let mut a = Configuration::default();
    let b = Configuration::default();

    a.discharge_cell(Cell::Cell18);
    assert_ne!(a, b);
}

#[test]
fn test_eq_true() {
    let mut a = Configuration::default();
    let mut b = Configuration::default();

    a.set_ov_comp_voltage(4_200_000).unwrap();
    b.set_ov_comp_voltage(4_200_000).unwrap();
    assert_eq!(a, b);
}

/// Asserts that all register slots, except one, match the default values
fn assert_default(except: usize, config: &Configuration) {
    let mut actual = [0u8; 12];
    actual[..6].clone_from_slice(&config.register_a);
    actual[6..].clone_from_slice(&config.register_b);

    let mut default = [0u8; 12];
    default[..6].clone_from_slice(&Configuration::default().register_a);
    default[6..].clone_from_slice(&Configuration::default().register_b);

    actual[except] = default[except];
    assert_eq!(actual, default);
}

/// Enables pull-down of all GPIO pins in register A
fn enable_all_gpio_pull_down_a(config: &mut Configuration) {
    config.enable_gpio_pull_down(GPIO::GPIO1);
    config.enable_gpio_pull_down(GPIO::GPIO2);
    config.enable_gpio_pull_down(GPIO::GPIO3);
    config.enable_gpio_pull_down(GPIO::GPIO4);
    config.enable_gpio_pull_down(GPIO::GPIO5);
}

/// Enables pull-down of all GPIO pins in register B
fn enable_all_gpio_pull_down_b(config: &mut Configuration) {
    config.enable_gpio_pull_down(GPIO::GPIO6);
    config.enable_gpio_pull_down(GPIO::GPIO7);
    config.enable_gpio_pull_down(GPIO::GPIO8);
    config.enable_gpio_pull_down(GPIO::GPIO9);
}
