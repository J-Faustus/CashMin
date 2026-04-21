//! # Submodules
//! this is just an organizational tool to make the main file shorter by removing unnecessary header code

pub mod Frobenius;
pub mod command;
pub mod math;
#[cfg(test)]
pub mod test;
pub mod texts;
pub mod utilities;

pub mod all {
    #![allow(unused_imports)]
    pub use super::Frobenius::*;
    pub use super::math::*;
    pub use super::utilities::*;
    pub use super::texts::*;
}

#[derive(Clone, Copy)]
pub enum CheckLevel {
    Optimal,
    Debt,
    Neither,
}

