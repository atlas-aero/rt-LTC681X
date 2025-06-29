//! Tests for static configuration of each device
use crate::ltc6810::LTC6810;
use crate::ltc6811::LTC6811;
use crate::ltc6812::LTC6812;
use crate::ltc6813::LTC6813;
use crate::monitor::{ChannelIndex, ChannelType, DeviceTypes, GroupedRegisterIndex, RegisterAddress, RegisterLocator};
use crate::{ltc6810, ltc6811, ltc6812, ltc6813};
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn test_ltc6813_grouped_index() {
    let mut aux = vec![];
    let mut cells = vec![];

    aux.push(ltc6813::Register::AuxiliaryA.to_index());
    aux.push(ltc6813::Register::AuxiliaryB.to_index());
    aux.push(ltc6813::Register::AuxiliaryC.to_index());
    aux.push(ltc6813::Register::AuxiliaryD.to_index());

    cells.push(ltc6813::Register::CellVoltageA.to_index());
    cells.push(ltc6813::Register::CellVoltageB.to_index());
    cells.push(ltc6813::Register::CellVoltageC.to_index());
    cells.push(ltc6813::Register::CellVoltageD.to_index());
    cells.push(ltc6813::Register::CellVoltageE.to_index());
    cells.push(ltc6813::Register::CellVoltageF.to_index());

    assert_eq!(vec! {0, 1, 2, 3}, aux);
    assert_eq!(vec! {0, 1, 2, 3, 4, 5}, cells);
}

#[test]
fn test_ltc6812_grouped_index() {
    let mut aux = vec![];
    let mut cells = vec![];

    aux.push(ltc6812::Register::AuxiliaryA.to_index());
    aux.push(ltc6812::Register::AuxiliaryB.to_index());
    aux.push(ltc6812::Register::AuxiliaryC.to_index());
    aux.push(ltc6812::Register::AuxiliaryD.to_index());

    cells.push(ltc6812::Register::CellVoltageA.to_index());
    cells.push(ltc6812::Register::CellVoltageB.to_index());
    cells.push(ltc6812::Register::CellVoltageC.to_index());
    cells.push(ltc6812::Register::CellVoltageD.to_index());
    cells.push(ltc6812::Register::CellVoltageE.to_index());

    assert_eq!(vec! {0, 1, 2, 3}, aux);
    assert_eq!(vec! {0, 1, 2, 3, 4}, cells);
}

#[test]
fn test_ltc6811_grouped_index() {
    let mut aux = vec![];
    let mut cells = vec![];

    aux.push(ltc6811::Register::AuxiliaryA.to_index());
    aux.push(ltc6811::Register::AuxiliaryB.to_index());

    cells.push(ltc6811::Register::CellVoltageA.to_index());
    cells.push(ltc6811::Register::CellVoltageB.to_index());
    cells.push(ltc6811::Register::CellVoltageC.to_index());
    cells.push(ltc6811::Register::CellVoltageD.to_index());

    assert_eq!(vec! {0, 1}, aux);
    assert_eq!(vec! {0, 1, 2, 3}, cells);
}

#[test]
fn test_ltc6810_grouped_index() {
    let mut aux = vec![];
    let mut cells = vec![];

    aux.push(ltc6810::Register::AuxiliaryA.to_index());
    aux.push(ltc6810::Register::AuxiliaryB.to_index());

    cells.push(ltc6810::Register::CellVoltageA.to_index());
    cells.push(ltc6810::Register::CellVoltageB.to_index());

    assert_eq!(vec! {0, 1}, aux);
    assert_eq!(vec! {0, 1}, cells);
}

#[test]
fn test_ltc6813_cell_register_locations_all() {
    let locations = ltc6813::CellSelection::All.get_locations();
    assert_cell_channel_mappings(
        ltc6813::CellSelection::All.get_locations().collect(),
        vec![
            ltc6813::Channel::Cell1,
            ltc6813::Channel::Cell2,
            ltc6813::Channel::Cell3,
            ltc6813::Channel::Cell4,
            ltc6813::Channel::Cell5,
            ltc6813::Channel::Cell6,
            ltc6813::Channel::Cell7,
            ltc6813::Channel::Cell8,
            ltc6813::Channel::Cell9,
            ltc6813::Channel::Cell10,
            ltc6813::Channel::Cell11,
            ltc6813::Channel::Cell12,
            ltc6813::Channel::Cell13,
            ltc6813::Channel::Cell14,
            ltc6813::Channel::Cell15,
            ltc6813::Channel::Cell16,
            ltc6813::Channel::Cell17,
            ltc6813::Channel::Cell18,
        ]
    );
    assert_cell_register_locations(locations.collect());
}

#[test]
fn test_ltc6813_cell_register_locations_groups() {
    let locations = ltc6813::CellSelection::Group1
        .get_locations()
        .chain(ltc6813::CellSelection::Group2.get_locations())
        .chain(ltc6813::CellSelection::Group3.get_locations())
        .chain(ltc6813::CellSelection::Group4.get_locations())
        .chain(ltc6813::CellSelection::Group5.get_locations())
        .chain(ltc6813::CellSelection::Group6.get_locations());

    assert_cell_register_locations(locations.collect());

    assert_eq!(3, ltc6813::CellSelection::Group1.get_locations().len());
    assert_eq!(3, ltc6813::CellSelection::Group2.get_locations().len());
    assert_eq!(3, ltc6813::CellSelection::Group3.get_locations().len());
    assert_eq!(3, ltc6813::CellSelection::Group4.get_locations().len());
    assert_eq!(3, ltc6813::CellSelection::Group5.get_locations().len());
    assert_eq!(3, ltc6813::CellSelection::Group6.get_locations().len());
}

#[test]
fn test_ltc6812_cell_register_locations_all() {
    let locations = ltc6812::CellSelection::All.get_locations();
    assert_cell_channel_mappings(
        ltc6812::CellSelection::All.get_locations().collect(),
        vec![
            ltc6812::Channel::Cell1,
            ltc6812::Channel::Cell2,
            ltc6812::Channel::Cell3,
            ltc6812::Channel::Cell4,
            ltc6812::Channel::Cell5,
            ltc6812::Channel::Cell6,
            ltc6812::Channel::Cell7,
            ltc6812::Channel::Cell8,
            ltc6812::Channel::Cell9,
            ltc6812::Channel::Cell10,
            ltc6812::Channel::Cell11,
            ltc6812::Channel::Cell12,
            ltc6812::Channel::Cell13,
            ltc6812::Channel::Cell14,
            ltc6812::Channel::Cell15,
        ]
    );
    assert_cell_register_locations(locations.collect());
}

#[test]
fn test_ltc6812_cell_register_locations_groups() {
    let locations = ltc6812::CellSelection::Group1
        .get_locations()
        .chain(ltc6812::CellSelection::Group2.get_locations())
        .chain(ltc6812::CellSelection::Group3.get_locations())
        .chain(ltc6812::CellSelection::Group4.get_locations())
        .chain(ltc6812::CellSelection::Group5.get_locations());

    assert_cell_register_locations(locations.collect());

    assert_eq!(3, ltc6812::CellSelection::Group1.get_locations().len());
    assert_eq!(3, ltc6812::CellSelection::Group2.get_locations().len());
    assert_eq!(3, ltc6812::CellSelection::Group3.get_locations().len());
    assert_eq!(3, ltc6812::CellSelection::Group4.get_locations().len());
    assert_eq!(3, ltc6812::CellSelection::Group5.get_locations().len());
}

#[test]
fn test_ltc6811_cell_register_locations_all() {
    let locations = ltc6811::CellSelection::All.get_locations();
    assert_cell_channel_mappings(
        ltc6811::CellSelection::All.get_locations().collect(),
        vec![
            ltc6811::Channel::Cell1,
            ltc6811::Channel::Cell2,
            ltc6811::Channel::Cell3,
            ltc6811::Channel::Cell4,
            ltc6811::Channel::Cell5,
            ltc6811::Channel::Cell6,
            ltc6811::Channel::Cell7,
            ltc6811::Channel::Cell8,
            ltc6811::Channel::Cell9,
            ltc6811::Channel::Cell10,
            ltc6811::Channel::Cell11,
            ltc6811::Channel::Cell12,
        ]
    );
    assert_cell_register_locations(locations.collect());
}

#[test]
fn test_ltc6811_cell_register_locations_groups() {
    let locations = ltc6811::CellSelection::Pair1
        .get_locations()
        .chain(ltc6811::CellSelection::Pair2.get_locations())
        .chain(ltc6811::CellSelection::Pair3.get_locations())
        .chain(ltc6811::CellSelection::Pair4.get_locations())
        .chain(ltc6811::CellSelection::Pair5.get_locations())
        .chain(ltc6811::CellSelection::Pair6.get_locations());

    assert_cell_register_locations(locations.collect());

    assert_eq!(2, ltc6811::CellSelection::Pair1.get_locations().len());
    assert_eq!(2, ltc6811::CellSelection::Pair2.get_locations().len());
    assert_eq!(2, ltc6811::CellSelection::Pair3.get_locations().len());
    assert_eq!(2, ltc6811::CellSelection::Pair4.get_locations().len());
    assert_eq!(2, ltc6811::CellSelection::Pair5.get_locations().len());
    assert_eq!(2, ltc6811::CellSelection::Pair6.get_locations().len());
}

#[test]
fn test_ltc6811_cell_selection_pairs() {
    assert_cell_channel_mappings(
        ltc6811::CellSelection::Pair1.get_locations().collect(),
        vec![ltc6811::Channel::Cell1, ltc6811::Channel::Cell7],
    );
    assert_cell_channel_mappings(
        ltc6811::CellSelection::Pair2.get_locations().collect(),
        vec![ltc6811::Channel::Cell2, ltc6811::Channel::Cell8],
    );
    assert_cell_channel_mappings(
        ltc6811::CellSelection::Pair3.get_locations().collect(),
        vec![ltc6811::Channel::Cell3, ltc6811::Channel::Cell9],
    );
    assert_cell_channel_mappings(
        ltc6811::CellSelection::Pair4.get_locations().collect(),
        vec![ltc6811::Channel::Cell4, ltc6811::Channel::Cell10],
    );
    assert_cell_channel_mappings(
        ltc6811::CellSelection::Pair5.get_locations().collect(),
        vec![ltc6811::Channel::Cell5, ltc6811::Channel::Cell11],
    );
    assert_cell_channel_mappings(
        ltc6811::CellSelection::Pair6.get_locations().collect(),
        vec![ltc6811::Channel::Cell6, ltc6811::Channel::Cell12],
    );
}

#[test]
fn test_ltc6810_cell_register_locations_all() {
    let locations = ltc6810::CellSelection::All.get_locations();
    assert_cell_channel_mappings(
        ltc6810::CellSelection::All.get_locations().collect(),
        vec![
            ltc6810::Channel::Cell1,
            ltc6810::Channel::Cell2,
            ltc6810::Channel::Cell3,
            ltc6810::Channel::Cell4,
            ltc6810::Channel::Cell5,
            ltc6810::Channel::Cell6,
        ]
    );
    assert_cell_register_locations(locations.collect());
}

#[test]
fn test_ltc6810_cell_register_locations_groups() {
    let locations = ltc6810::CellSelection::Cell1
        .get_locations()
        .chain(ltc6810::CellSelection::Cell2.get_locations())
        .chain(ltc6810::CellSelection::Cell3.get_locations())
        .chain(ltc6810::CellSelection::Cell4.get_locations())
        .chain(ltc6810::CellSelection::Cell5.get_locations())
        .chain(ltc6810::CellSelection::Cell6.get_locations());

    assert_cell_register_locations(locations.collect());

    assert_eq!(1, ltc6810::CellSelection::Cell1.get_locations().len());
    assert_eq!(1, ltc6810::CellSelection::Cell2.get_locations().len());
    assert_eq!(1, ltc6810::CellSelection::Cell3.get_locations().len());
    assert_eq!(1, ltc6810::CellSelection::Cell4.get_locations().len());
    assert_eq!(1, ltc6810::CellSelection::Cell5.get_locations().len());
    assert_eq!(1, ltc6810::CellSelection::Cell6.get_locations().len());
}

#[test]
fn test_ltc6813_gpio_register_locations_all() {
    let locations = ltc6813::GPIOSelection::All.get_locations();
    assert_gpio_register_locations(locations.collect());

    let locations: Vec<&RegisterAddress<LTC6813>> = ltc6813::GPIOSelection::All.get_locations().collect();
    assert_eq!(ltc6813::Channel::SecondReference, locations[9].channel);
}

#[test]
fn test_ltc6813_gpio_register_locations_groups() {
    let locations = ltc6813::GPIOSelection::Group1
        .get_locations()
        .chain(ltc6813::GPIOSelection::Group2.get_locations())
        .chain(ltc6813::GPIOSelection::Group3.get_locations())
        .chain(ltc6813::GPIOSelection::Group4.get_locations())
        .chain(ltc6813::GPIOSelection::Group5.get_locations())
        .chain(ltc6813::GPIOSelection::Group6.get_locations());

    assert_gpio_register_locations(locations.collect());

    assert_eq!(2, ltc6813::GPIOSelection::Group1.get_locations().len());
    assert_eq!(2, ltc6813::GPIOSelection::Group2.get_locations().len());
    assert_eq!(2, ltc6813::GPIOSelection::Group3.get_locations().len());
    assert_eq!(2, ltc6813::GPIOSelection::Group4.get_locations().len());
    assert_eq!(1, ltc6813::GPIOSelection::Group5.get_locations().len());

    let second_ref: Vec<&RegisterAddress<LTC6813>> = ltc6813::GPIOSelection::Group6.get_locations().collect();
    assert_eq!(1, second_ref.len());
    assert_eq!(ltc6813::Channel::SecondReference, second_ref[0].channel);
}

#[test]
fn test_ltc6812_gpio_register_locations_all() {
    let locations = ltc6812::GPIOSelection::All.get_locations();
    assert_gpio_register_locations(locations.collect());

    let locations: Vec<&RegisterAddress<LTC6812>> = ltc6812::GPIOSelection::All.get_locations().collect();
    assert_eq!(ltc6812::Channel::SecondReference, locations[9].channel);
}

#[test]
fn test_ltc6812_gpio_register_locations_groups() {
    let locations = ltc6812::GPIOSelection::Group1
        .get_locations()
        .chain(ltc6812::GPIOSelection::Group2.get_locations())
        .chain(ltc6812::GPIOSelection::Group3.get_locations())
        .chain(ltc6812::GPIOSelection::Group4.get_locations())
        .chain(ltc6812::GPIOSelection::Group5.get_locations())
        .chain(ltc6812::GPIOSelection::Group6.get_locations());

    assert_gpio_register_locations(locations.collect());

    assert_eq!(2, ltc6812::GPIOSelection::Group1.get_locations().len());
    assert_eq!(2, ltc6812::GPIOSelection::Group2.get_locations().len());
    assert_eq!(2, ltc6812::GPIOSelection::Group3.get_locations().len());
    assert_eq!(2, ltc6812::GPIOSelection::Group4.get_locations().len());
    assert_eq!(1, ltc6812::GPIOSelection::Group5.get_locations().len());

    let second_ref: Vec<&RegisterAddress<LTC6812>> = ltc6812::GPIOSelection::Group6.get_locations().collect();
    assert_eq!(1, second_ref.len());
    assert_eq!(ltc6812::Channel::SecondReference, second_ref[0].channel);
}

#[test]
fn test_ltc6811_gpio_register_locations_all() {
    let locations = ltc6811::GPIOSelection::All.get_locations();
    assert_gpio_register_locations(locations.collect());

    let locations: Vec<&RegisterAddress<LTC6811>> = ltc6811::GPIOSelection::All.get_locations().collect();
    assert_eq!(ltc6811::Channel::SecondReference, locations[5].channel);
}

#[test]
fn test_ltc6811_gpio_register_locations_groups() {
    let locations = ltc6811::GPIOSelection::GPIO1
        .get_locations()
        .chain(ltc6811::GPIOSelection::GPIO2.get_locations())
        .chain(ltc6811::GPIOSelection::GPIO3.get_locations())
        .chain(ltc6811::GPIOSelection::GPIO4.get_locations())
        .chain(ltc6811::GPIOSelection::GPIO5.get_locations())
        .chain(ltc6811::GPIOSelection::SecondReference.get_locations());

    assert_gpio_register_locations(locations.collect());

    assert_eq!(1, ltc6811::GPIOSelection::GPIO1.get_locations().len());
    assert_eq!(1, ltc6811::GPIOSelection::GPIO2.get_locations().len());
    assert_eq!(1, ltc6811::GPIOSelection::GPIO3.get_locations().len());
    assert_eq!(1, ltc6811::GPIOSelection::GPIO4.get_locations().len());
    assert_eq!(1, ltc6811::GPIOSelection::GPIO5.get_locations().len());

    let second_ref: Vec<&RegisterAddress<LTC6811>> = ltc6811::GPIOSelection::SecondReference.get_locations().collect();
    assert_eq!(1, second_ref.len());
    assert_eq!(ltc6811::Channel::SecondReference, second_ref[0].channel);
}

/// Array representation of correct GPIO register locations
const LTC6810_CORRECT_GPIO_LOCATIONS: [[usize; 3]; 4] = [[0, 0, 1], [1, 0, 2], [2, 1, 0], [3, 1, 1]];

#[test]
fn test_ltc6810_gpio_register_locations_all() {
    let locations: Vec<&RegisterAddress<LTC6810>> = ltc6810::GPIOSelection::GPIO1
        .get_locations()
        .chain(ltc6810::GPIOSelection::GPIO2.get_locations())
        .chain(ltc6810::GPIOSelection::GPIO3.get_locations())
        .chain(ltc6810::GPIOSelection::GPIO4.get_locations())
        .collect();

    assert_eq!(4, locations.len());
    assert_eq!(LTC6810_CORRECT_GPIO_LOCATIONS, locations_to_array(locations)[0..4]);

    let second_ref: Vec<&RegisterAddress<LTC6810>> = ltc6810::GPIOSelection::SecondReference.get_locations().collect();
    assert_eq!(1, second_ref.len());
    assert_eq!(ltc6810::Channel::SecondReference, second_ref[0].channel);

    let second_ref: Vec<&RegisterAddress<LTC6810>> = ltc6810::GPIOSelection::S0.get_locations().collect();
    assert_eq!(1, second_ref.len());
    assert_eq!(ltc6810::Channel::S0, second_ref[0].channel);
}

#[test]
fn test_ltc6810_gpio_register_locations_groups() {
    let mut locations: Vec<&RegisterAddress<LTC6810>> = ltc6810::GPIOSelection::All.get_locations().collect();
    assert_eq!(6, locations.len());

    assert_eq!(ltc6810::Channel::S0, locations[0].channel);
    assert_eq!(ltc6810::Register::AuxiliaryA, locations[0].register);
    assert_eq!(0, locations[0].slot);

    assert_eq!(ltc6810::Channel::SecondReference, locations[5].channel);
    assert_eq!(ltc6810::Register::AuxiliaryB, locations[5].register);
    assert_eq!(2, locations[5].slot);

    locations.remove(0);
    assert_eq!(LTC6810_CORRECT_GPIO_LOCATIONS, locations_to_array(locations)[0..4]);
}

/// Array representation of correct cell register locations
const CORRECT_CELL_LOCATIONS: [[usize; 3]; 18] = [
    [0, 0, 0],
    [1, 0, 1],
    [2, 0, 2],
    [3, 1, 0],
    [4, 1, 1],
    [5, 1, 2],
    [6, 2, 0],
    [7, 2, 1],
    [8, 2, 2],
    [9, 3, 0],
    [10, 3, 1],
    [11, 3, 2],
    [12, 4, 0],
    [13, 4, 1],
    [14, 4, 2],
    [15, 5, 0],
    [16, 5, 1],
    [17, 5, 2],
];

/// Checks that the channels from the listed RegisterAddresses match expected.
fn assert_cell_channel_mappings<T: DeviceTypes>(actual: Vec<&RegisterAddress<T>>, expected: Vec<T::Channel>)
where
    T::Channel: PartialEq + std::fmt::Debug,
{
    assert_eq!(actual.len(), expected.len());
    for i in 0..actual.len() {
        assert_eq!(actual[i].channel, expected[i]);
    }
}

fn assert_cell_register_locations<T: DeviceTypes>(locations: Vec<&RegisterAddress<T>>) {
    assert_eq!(T::CELL_COUNT, locations.len());
    let result = locations_to_array(locations);
    assert_eq!(CORRECT_CELL_LOCATIONS[0..T::CELL_COUNT], result[0..T::CELL_COUNT]);
}

/// Array representation of correct GPIO register locations
const CORRECT_GPIO_LOCATIONS: [[usize; 3]; 9] = [
    [0, 0, 0],
    [1, 0, 1],
    [2, 0, 2],
    [3, 1, 0],
    [4, 1, 1],
    [5, 2, 0],
    [6, 2, 1],
    [7, 2, 2],
    [8, 3, 0],
];

fn assert_gpio_register_locations<T: DeviceTypes>(locations: Vec<&RegisterAddress<T>>) {
    assert_eq!(T::GPIO_COUNT + 1, locations.len());
    let result = locations_to_array(locations);
    assert_eq!(CORRECT_GPIO_LOCATIONS[0..T::GPIO_COUNT], result[0..T::GPIO_COUNT]);
}

fn locations_to_array<T: DeviceTypes>(locations: Vec<&RegisterAddress<T>>) -> [[usize; 3]; 18] {
    let mut result: [[usize; 3]; 18] = [[0, 0, 0]; 18];

    for location in locations {
        let channel_type = location.channel.into();
        let channel_index = match channel_type {
            ChannelType::Cell => location.channel.to_cell_index().unwrap(),
            ChannelType::GPIO => location.channel.to_gpio_index().unwrap(),
            ChannelType::Reference => 17,
        };
        result[channel_index] = [channel_index, location.register.to_index(), location.slot];
    }

    result
}
