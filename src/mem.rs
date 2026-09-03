use std::ops::Index;
use std::ops::IndexMut;

/// The memory subsystem - a flat 64KB address space.
///
/// Port of `m6502::Mem` from m6502.h. Unlike the C++ version there is no
/// way to read outside the bounds: addresses are `u16`, which cover
/// exactly the 64KB address space.
#[derive(Clone, Debug)]
pub struct Mem {
    pub data: [u8; Mem::MAX_MEM],
}

impl Default for Mem {
    fn default() -> Self {
        Mem::new()
    }
}

impl Mem {
    pub const MAX_MEM: usize = 1024 * 64;

    pub fn new() -> Self {
        Mem {
            data: [0; Mem::MAX_MEM],
        }
    }

    pub fn initialise(&mut self) {
        self.data = [0; Mem::MAX_MEM];
    }
}

/// read 1 byte
impl Index<u16> for Mem {
    type Output = u8;

    fn index(&self, address: u16) -> &u8 {
        &self.data[address as usize]
    }
}

/// write 1 byte
impl IndexMut<u16> for Mem {
    fn index_mut(&mut self, address: u16) -> &mut u8 {
        &mut self.data[address as usize]
    }
}
