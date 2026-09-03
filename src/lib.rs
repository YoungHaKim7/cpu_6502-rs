//! Rust port of the C++ 6502 emulator.
//!
//! Written during the youtube video: https://youtu.be/qJgsuQoy9bc
//! 6502 reference: http://www.obelisk.me.uk/6502/

pub mod cpu;
pub mod mem;

pub use cpu::{Cpu, StatusFlags};
pub use mem::Mem;
