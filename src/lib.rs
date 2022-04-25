#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]

mod ltc6813;
pub mod monitor;
pub(crate) mod pec15;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;
