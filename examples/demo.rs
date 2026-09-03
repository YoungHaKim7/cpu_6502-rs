//! A small demo program for the m6502 CPU, in the spirit of the original
//! youtube video (https://youtu.be/qJgsuQoy9bc).
//!
//! The .prg style program (first two bytes = load address, like a C64 file):
//!
//! ```text
//! * = $0400
//! LDA #$42      load the A register with 0x42
//! JSR $0500     call a subroutine at $0500
//! STA $10       back from the subroutine: store A at zero page $10
//!
//! * = $0500
//! INX           the subroutine increments X...
//! INX           ...twice
//! RTS           and returns
//! ```
//!
//! Run it with: cargo run --example demo

use cpu_6502_rs::{Cpu, Mem};

const PRG: &[u8] = &[
    0x00, 0x04, // load address: $0400
    0xA9, 0x42, // LDA #$42
    0x20, 0x00, 0x05, // JSR $0500
    0x85, 0x10, // STA $10
          // subroutine at $0500 (gap is zero-filled memory)
];

const SUBROUTINE: &[u8] = &[0xE8, 0xE8, 0x60]; // INX, INX, RTS

fn main() {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();

    cpu.reset(&mut mem);

    // load the program and jump to its start address
    let start_address = cpu.load_prg(PRG, &mut mem);
    cpu.pc = start_address;

    // poke the subroutine in at $0500
    for (i, &byte) in SUBROUTINE.iter().enumerate() {
        mem[0x0500 + i as u16] = byte;
    }

    println!("Running...");
    // LDA(2) + JSR(6) + INX(2) + INX(2) + RTS(6) + STA(3) = 21 cycles
    let total_cycles = cpu.execute(2 + 6 + 2 + 2 + 6 + 3, &mut mem);

    println!("Done. Cycles used: {}", total_cycles);
    cpu.print_status();
    println!("mem[0x0010] = {:#04x} (should be 0x42)", mem[0x0010]);
    println!("X = {} (the subroutine incremented it twice)", cpu.x);
}
