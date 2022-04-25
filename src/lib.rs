#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]

pub mod ltc6810;
pub mod ltc6811;
pub mod ltc6812;
pub mod ltc6813;
pub mod monitor;

pub(crate) mod commands;
pub(crate) mod pec15;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;
