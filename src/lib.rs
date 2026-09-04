//! Rust port of the C++ 6502 emulator.
//!
//! Written during the youtube video: https://youtu.be/qJgsuQoy9bc
//! 6502 reference: http://www.obelisk.me.uk/6502/
//!
//! # 6502 CPU Emulator
//!
//! A Rust port of a C++ MOS 6502 CPU emulator.
//!
//! This crate implements a software model of the 6502 processor, including
//! its CPU registers, processor-status flags, memory accesses, stack,
//! addressing modes, instruction decoding, instruction execution, and
//! approximate cycle accounting.
//!
//! The implementation is divided into two major modules:
//!
//! ```text
//! src/
//! ├── lib.rs       <- crate-level documentation and public API
//! ├── cpu.rs       <- 6502 CPU implementation
//! └── mem.rs       <- 64 KiB memory implementation
//! ```
//!
//! The relationship between the two modules is:
//!
//! ```text
//!                    6502 Emulator
//!                         │
//!              ┌──────────┴──────────┐
//!              │                     │
//!              ▼                     ▼
//!            Cpu                   Mem
//!              │                     │
//!              │  read/write         │
//!              ├────────────────────►│
//!              │                     │
//!              │     64 KiB          │
//!              │   address space     │
//!              │                     │
//!              └─────────────────────┘
//! ```
//!
//! The [`Cpu`] represents the processor state and behavior, while [`Mem`]
//! represents the memory connected to the processor.
//!
//! ## The MOS 6502
//!
//! The MOS Technology 6502 is an 8-bit microprocessor with a 16-bit address
//! space.
//!
//! An 8-bit CPU generally operates on values from:
//!
//! ```text
//! 0x00 ..= 0xFF
//! ```
//!
//! A 16-bit address allows the processor to address:
//!
//! ```text
//! 2^16 = 65,536 bytes
//! ```
//!
//! Therefore, the 6502's address space is:
//!
//! ```text
//! 0x0000 ..= 0xFFFF
//! ```
//!
//! This emulator represents individual memory values with [`u8`] and memory
//! addresses with [`u16`].
//!
//! ## CPU registers
//!
//! The [`Cpu`] structure models the primary programmer-visible CPU state:
//!
//! ```text
//! +-------------------------------+
//! |             CPU               |
//! +-------------------------------+
//! | PC  : 16-bit Program Counter  |
//! | SP  :  8-bit Stack Pointer    |
//! | A   :  8-bit Accumulator      |
//! | X   :  8-bit X Register       |
//! | Y   :  8-bit Y Register       |
//! | P   : Processor Status        |
//! +-------------------------------+
//! ```
//!
//! In this implementation, the processor status register is represented by
//! [`StatusFlags`] rather than by a single `u8`.
//!
//! The [`StatusFlags`] structure contains one Boolean value for each status
//! bit. The implementation can convert between the Boolean representation
//! and the 6502's packed processor-status byte using
//! [`StatusFlags::to_byte`] and [`StatusFlags::from_byte`].
//!
//! ## Processor status register
//!
//! The 6502 processor-status byte contains eight individual bits:
//!
//! ```text
//! Bit:  7   6   5   4   3   2   1   0
//!       N   V   U   B   D   I   Z   C
//! ```
//!
//! Where:
//!
//! ```text
//! N = Negative
//! V = Overflow
//! U = Unused
//! B = Break
//! D = Decimal mode
//! I = Interrupt Disable
//! Z = Zero
//! C = Carry
//! ```
//!
//! The Rust implementation stores these bits as Boolean fields:
//!
//! ```text
//! StatusFlags
//! ├── c      Carry
//! ├── z      Zero
//! ├── i      Interrupt Disable
//! ├── d      Decimal Mode
//! ├── b      Break
//! ├── unused Unused
//! ├── v      Overflow
//! └── n      Negative
//! ```
//!
//! The implementation deliberately keeps the individual Boolean values as
//! the source of truth and performs bit packing only when a processor-status
//! byte is required.
//!
//! See [`StatusFlags::to_byte`] and [`StatusFlags::from_byte`].
//!
//! ## Memory
//!
//! The [`Mem`] type represents the emulator's flat 64 KiB address space.
//!
//! Its underlying storage is:
//!
//! ```text
//! [u8; 1024 * 64]
//! ```
//!
//! In other words:
//!
//! ```text
//! 64 KiB = 65,536 bytes
//! ```
//!
//! Memory can be accessed using a `u16` address:
//!
//! ```rs
//! memory[0x0000]
//! memory[0x1234]
//! memory[0xFFFF]
//! ```
//!
//! The implementation uses Rust's [`Index`] and [`IndexMut`] traits so that
//! memory access resembles an ordinary array access.
//!
//! Conceptually:
//!
//! ```text
//! CPU address
//!      │
//!      │ u16
//!      ▼
//! +----------+
//! |   Mem    |
//! +----------+
//!      │
//!      ▼
//!   one u8
//! ```
//!
//! Because every possible `u16` value corresponds to exactly one location in
//! the 64 KiB array, the address type naturally describes the complete
//! address space.
//!
//! ## Fetch / Decode / Execute
//!
//! The central operation of the emulator is [`Cpu::execute`].
//!
//! A real processor repeatedly performs a fetch/decode/execute sequence.
//! This emulator models that process in software.
//!
//! ```text
//!                 ┌─────────────┐
//!                 │             │
//!                 │   Execute   │
//!                 │             │
//!                 └──────┬──────┘
//!                        │
//!                        ▼
//!                 ┌─────────────┐
//!                 │    Fetch    │
//!                 │    opcode   │
//!                 └──────┬──────┘
//!                        │
//!                        ▼
//!                 ┌─────────────┐
//!                 │    Decode   │
//!                 │    opcode   │
//!                 └──────┬──────┘
//!                        │
//!                        ▼
//!                 ┌─────────────┐
//!                 │   Execute   │
//!                 │ instruction│
//!                 └──────┬──────┘
//!                        │
//!                        └──────────────► next instruction
//! ```
//!
//! The implementation first fetches an opcode from the address currently
//! stored in the program counter.
//!
//! The program counter is then advanced, and the opcode is matched against
//! the instruction constants defined by [`Cpu`].
//!
//! For example:
//!
//! ```rs
//! Self::INS_LDA_IM => {
//!     self.a = self.fetch_byte(&mut cycles, memory);
//!     self.set_zero_and_negative_flags(self.a);
//! }
//! ```
//!
//! This represents an immediate `LDA` instruction:
//!
//! ```text
//! LDA #$42
//! ```
//!
//! Conceptually, the CPU performs:
//!
//! ```text
//! PC
//! │
//! ▼
//! +------+-------+
//! | A9   |  42   |
//! +------+-------+
//!   │        │
//!   │        └── operand
//!   └─────────── opcode
//! ```
//!
//! The opcode is fetched first, followed by the operand.
//!
//! ## Instruction opcodes
//!
//! A 6502 instruction is encoded as one or more bytes.
//!
//! The first byte is the opcode. The opcode identifies both the operation
//! and, implicitly, its addressing mode.
//!
//! The implementation defines opcode constants such as:
//!
//! ```rs
//! Cpu::INS_LDA_IM
//! Cpu::INS_LDA_ZP
//! Cpu::INS_LDA_ZPX
//! Cpu::INS_LDA_ABS
//! Cpu::INS_LDA_ABSX
//! Cpu::INS_LDA_ABSY
//! Cpu::INS_LDA_INDX
//! Cpu::INS_LDA_INDY
//! ```
//!
//! These represent different encodings of the `LDA` instruction.
//!
//! The same approach is used for arithmetic, logical, branching, stack,
//! comparison, shifting, and control-flow instructions.
//!
//! ## Addressing modes
//!
//! One of the most important concepts in a 6502 emulator is the addressing
//! mode.
//!
//! An instruction specifies an operation, but the addressing mode specifies
//! where the operand comes from.
//!
//! This implementation contains functions for several addressing modes,
//! including:
//!
//! - Zero Page
//! - Zero Page,X
//! - Zero Page,Y
//! - Absolute
//! - Absolute,X
//! - Absolute,Y
//! - Indexed Indirect, `(Indirect,X)`
//! - Indirect Indexed, `(Indirect),Y`
//!
//! The addressing-mode functions are responsible for calculating the
//! effective memory address used by an instruction.
//!
//! For example:
//!
//! ```rs
//! let address = self.addr_absolute(&mut cycles, memory);
//! ```
//!
//! obtains a 16-bit absolute address from the instruction stream.
//!
//! Indexed addressing adds an index register to a base address:
//!
//! ```text
//! effective address = base address + X
//! ```
//!
//! or:
//!
//! ```text
//! effective address = base address + Y
//! ```
//!
//! The implementation also checks whether an indexed address crosses a
//! 256-byte page boundary because page crossing can require an additional
//! CPU cycle for certain instructions.
//!
//! ## Zero Page
//!
//! The 6502's Zero Page occupies addresses:
//!
//! ```text
//! 0x0000 ..= 0x00FF
//! ```
//!
//! A zero-page address is represented by only one byte in the instruction.
//!
//! For example:
//!
//! ```text
//! LDA $20
//! ```
//!
//! uses `$20` as an address within the first 256 bytes of memory.
//!
//! Zero-page indexed addressing adds `X` or `Y` to that 8-bit address.
//!
//! The implementation uses [`u8::wrapping_add`] semantics through
//! `wrapping_add`, preserving the 6502's 8-bit wrapping behavior.
//!
//! ## Little-endian words
//!
//! The 6502 stores 16-bit addresses in little-endian order.
//!
//! A 16-bit value is therefore stored as:
//!
//! ```text
//! low byte
//! high byte
//! ```
//!
//! For example:
//!
//! ```text
//! address = 0x1234
//!
//! memory:
//!
//!  low byte  = 0x34
//!  high byte = 0x12
//! ```
//!
//! The implementation reconstructs such a word using:
//!
//! ```text
//! word = low_byte | (high_byte << 8)
//! ```
//!
//! This behavior is implemented by [`Cpu::fetch_word`] and
//! [`Cpu::read_word`].
//!
//! ## Program counter
//!
//! The program counter (`PC`) is a 16-bit register.
//!
//! It identifies the location from which the next instruction or operand
//! will be fetched.
//!
//! Fetching a byte performs three conceptual operations:
//!
//! ```text
//! 1. read memory[PC]
//! 2. increment PC
//! 3. consume one CPU cycle
//! ```
//!
//! The implementation uses [`u16::wrapping_add`] when advancing the program
//! counter so that the 16-bit address naturally wraps from `$FFFF` back to
//! `$0000`.
//!
//! ## Stack
//!
//! The 6502 stack resides in page `$01xx`:
//!
//! ```text
//! $0100 ..= $01FF
//! ```
//!
//! The stack pointer itself is only 8 bits wide.
//!
//! The full stack address is therefore constructed as:
//!
//! ```text
//! 0x0100 | SP
//! ```
//!
//! The [`Cpu::sp_to_address`] function performs this conversion.
//!
//! ```text
//! SP = 0xFF
//!
//! full address = 0x0100 | 0x00FF
//!              = 0x01FF
//! ```
//!
//! Stack operations include:
//!
//! - pushing bytes,
//! - popping bytes,
//! - pushing 16-bit values,
//! - popping 16-bit values,
//! - saving processor status,
//! - restoring processor status.
//!
//! These operations are required by instructions such as:
//!
//! ```text
//! PHA
//! PLA
//! PHP
//! PLP
//! JSR
//! RTS
//! BRK
//! RTI
//! ```
//!
//! ## Subroutines
//!
//! The 6502 uses [`JSR`] and [`RTS`] for subroutine calls and returns.
//!
//! `JSR` saves a return address on the stack and changes the program counter
//! to the subroutine address.
//!
//! `RTS` restores the saved address and resumes execution after the call.
//!
//! The implementation provides:
//!
//! ```text
//! push_pc_minus_one_to_stack()
//! pop_word_from_stack()
//! ```
//!
//! to model the required stack behavior.
//!
//! ## Interrupts and BRK
//!
//! The implementation also contains `BRK` and `RTI` instructions.
//!
//! `BRK` saves processor state on the stack and obtains a new program
//! counter from the interrupt vector.
//!
//! The interrupt vector used by this implementation is:
//!
//! ```text
//! $FFFE
//! ```
//!
//! `RTI` restores the saved processor status and program counter.
//!
//! ## Arithmetic
//!
//! The emulator implements `ADC` and `SBC`.
//!
//! `ADC` means:
//!
//! ```text
//! Add with Carry
//! ```
//!
//! The conceptual operation is:
//!
//! ```text
//! A = A + operand + C
//! ```
//!
//! where `C` is the Carry flag.
//!
//! The result is stored back into the accumulator and the Zero, Negative,
//! Carry, and Overflow flags are updated.
//!
//! The current implementation explicitly does not implement decimal-mode
//! arithmetic. Calling the internal `adc` operation while the Decimal flag
//! is set triggers an assertion:
//!
//! ```text
//! "haven't handled decimal mode!"
//! ```
//!
//! Therefore, this is an important implementation limitation to keep in
//! mind when using the emulator.
//!
//! ## Bitwise operations
//!
//! The implementation provides:
//!
//! ```text
//! AND
//! ORA
//! EOR
//! BIT
//! ```
//!
//! The accumulator is used by the logical operations.
//!
//! For example:
//!
//! ```text
//! A = A AND operand
//! ```
//!
//! After the operation, the Zero and Negative flags are updated.
//!
//! `BIT` is different because it tests bits in a memory operand and also
//! obtains the Negative and Overflow flags from the operand.
//!
//! ## Shifts and rotates
//!
//! The emulator implements:
//!
//! ```text
//! ASL  Arithmetic Shift Left
//! LSR  Logical Shift Right
//! ROL  Rotate Left
//! ROR  Rotate Right
//! ```
//!
//! These instructions demonstrate why the Carry flag is more than a simple
//! arithmetic flag: it participates directly in bit shifting and rotation.
//!
//! For example, a rotate-left operation conceptually performs:
//!
//! ```text
//!          +--- old Carry
//!          |
//!          v
//! +---+---+---+---+---+---+---+---+
//! | C | 7 | 6 | 5 | 4 | 3 | 2 | 1 |
//! +---+---+---+---+---+---+---+---+
//!                             |
//!                             v
//!                           new C
//! ```
//!
//! The implementation explicitly transfers the old Carry flag into bit 0
//! while transferring the old bit 7 into Carry.
//!
//! ## Branch instructions
//!
//! Conditional branch instructions use the processor-status flags.
//!
//! The implementation supports:
//!
//! ```text
//! BEQ  Branch if Equal
//! BNE  Branch if Not Equal
//! BCS  Branch if Carry Set
//! BCC  Branch if Carry Clear
//! BMI  Branch if Minus
//! BPL  Branch if Plus
//! BVC  Branch if Overflow Clear
//! BVS  Branch if Overflow Set
//! ```
//!
//! Branch offsets are signed 8-bit values.
//!
//! Conceptually:
//!
//! ```text
//! new PC = current PC + signed offset
//! ```
//!
//! The implementation also checks whether the branch crosses a 256-byte
//! memory page and accounts for the additional cycle.
//!
//! ## Cycle accounting
//!
//! The [`Cpu::execute`] function receives a requested number of cycles.
//!
//! Individual CPU operations subtract cycles from this counter.
//!
//! Conceptually:
//!
//! ```text
//! requested cycles
//!        │
//!        ▼
//! +----------------+
//! | cycle counter  |
//! +----------------+
//!        │
//!        ├── fetch opcode     -1
//!        ├── fetch operand    -1
//!        ├── memory read      -1
//!        ├── memory write     -1
//!        └── extra page cycle -1
//!        │
//!        ▼
//! remaining cycles
//! ```
//!
//! At the end, `execute` returns the number of cycles actually consumed.
//!
//! This allows the emulator to model not only the final register/memory
//! state but also the approximate timing behavior of instructions.
//!
//! ## Rust and the original C++ implementation
//!
//! This project is a port of a C++ implementation.
//!
//! One notable difference is the representation of processor flags.
//!
//! The original C++ implementation uses a bitfield/union representation,
//! while this Rust implementation uses individual Boolean fields and
//! explicit conversion functions.
//!
//! Another important Rust-specific feature is the use of the [`Index`] and
//! [`IndexMut`] traits for memory access.
//!
//! This allows code such as:
//!
//! ```rs
//! memory[address]
//! ```
//!
//! to represent a memory read and:
//!
//! ```rs
//! memory[address] = value;
//! ```
//!
//! to represent a memory write.
//!
//! ## Public API
//!
//! The implementation modules are publicly exposed:
//!
//! ```rs
//! pub mod cpu;
//! pub mod mem;
//! ```
//!
//! The most important types are also re-exported at the crate root:
//!
//! ```rs
//! pub use cpu::{Cpu, StatusFlags};
//! pub use mem::Mem;
//! ```
//!
//! Therefore, users can write:
//!
//! ```rs
//! use your_crate::{Cpu, Mem, StatusFlags};
//! ```
//!
//! rather than:
//!
//! ```rs
//! use your_crate::cpu::Cpu;
//! use your_crate::cpu::StatusFlags;
//! use your_crate::mem::Mem;
//! ```
//!
//! The re-exports make the main emulator types easier to discover and use.
//!
//! ## Basic usage
//!
//! A typical program using the emulator has the following conceptual
//! structure:
//!
//! ```rs
//! use your_crate::{Cpu, Mem};
//!
//! fn main() {
//!     // Create the 64 KiB memory subsystem.
//!     let mut memory = Mem::new();
//!
//!     // Create the CPU.
//!     let mut cpu = Cpu::new();
//!
//!     // Load or construct a program in memory.
//!     //
//!     // The exact program-loading mechanism depends on the program format.
//!
//!     // Reset the processor.
//!     cpu.reset(&mut memory);
//!
//!     // Execute some CPU cycles.
//!     cpu.execute(100, &mut memory);
//! }
//! ```
//!
//! ## Program loading
//!
//! The [`Cpu::load_prg`] function accepts a byte slice representing a
//! program image whose first two bytes specify the load address.
//!
//! The address is interpreted as little-endian:
//!
//! ```text
//! byte 0 = low byte
//! byte 1 = high byte
//! ```
//!
//! The remaining bytes are copied into memory beginning at that address.
//!
//! This is useful for loading small 6502 programs into the emulator.
//!
//! ## Reset
//!
//! [`Cpu::reset`] initializes the processor using the reset address
//! `$FFFC` represented by the current implementation.
//!
//! [`Cpu::reset_at`] provides an explicit reset address.
//!
//! Reset initializes the main CPU state and also clears the emulator's
//! memory through [`Mem::initialise`].
//!
//! This behavior is convenient for a standalone emulator/test environment,
//! although a complete hardware-accurate computer emulator may model reset
//! and external memory behavior differently.
//!
//! ## Implementation notes
//!
//! This emulator should be understood as a learning-oriented software model
//! of the 6502 rather than automatically assuming cycle-perfect hardware
//! compatibility.
//!
//! In particular, the source itself documents an implementation limitation
//! concerning decimal-mode arithmetic and a compatibility issue concerning
//! indirect `JMP` page-boundary behavior.
//!
//! When extending the emulator, these hardware-specific details should be
//! checked against a trusted 6502 reference before changing the
//! implementation.
//!
//! ## Useful 6502 references
//!
//! ### Obelisk 6502 Reference
//!
//! A very useful reference for learning the 6502 instruction set, opcodes,
//! addressing modes, flags, and instruction behavior:
//!
//! <http://www.obelisk.me.uk/6502/>
//!
//! ### 6502 Instruction Set
//!
//! When documenting an instruction in `cpu.rs`, compare its implementation
//! against an instruction reference for:
//!
//! - opcode,
//! - addressing mode,
//! - instruction length,
//! - affected flags,
//! - cycle count,
//! - page-boundary behavior.
//!
//! ### 6502.org
//!
//! A useful collection of 6502 technical information, documentation,
//! projects, and discussion:
//!
//! <https://www.6502.org/>
//!
//! ### Visual 6502
//!
//! Visual 6502 is particularly useful for understanding the actual
//! transistor-level implementation of the original NMOS 6502.
//!
//! <http://www.visual6502.org/>
//!
//! It is valuable when moving from an instruction-level emulator toward
//! understanding how the original hardware actually performs operations.
//!
//! ## Rust references
//!
//! ### Rust Reference
//!
//! The official Rust Reference explains the language semantics used by this
//! project:
//!
//! <https://doc.rust-lang.org/reference/>
//!
//! ### Rust standard library
//!
//! The standard-library documentation is especially useful for the traits
//! used by the memory subsystem:
//!
//! [`Index`]:
//! <https://doc.rust-lang.org/std/ops/trait.Index.html>
//!
//! [`IndexMut`]:
//! <https://doc.rust-lang.org/std/ops/trait.IndexMut.html>
//!
//! [`u8`]:
//! <https://doc.rust-lang.org/std/primitive.u8.html>
//!
//! [`u16`]:
//! <https://doc.rust-lang.org/std/primitive.u16.html>
//!
//! ## Original project / video
//!
//! The original emulator was implemented in C++ and demonstrated in the
//! associated YouTube video:
//!
//! <https://youtu.be/qJgsuQoy9bc>
//!
//! The Rust implementation preserves the overall organization and behavior
//! of the original while adapting the implementation to Rust's type system,
//! ownership model, traits, and standard-library facilities.
//!
//! ## Recommended reading order
//!
//! If you are studying this emulator to learn both Rust and CPU emulation,
//! the following order is useful:
//!
//! ```text
//! 1. lib.rs
//!      │
//!      │ Understand the architecture
//!      ▼
//! 2. mem.rs
//!      │
//!      │ Understand the 64 KiB memory model
//!      ▼
//! 3. StatusFlags
//!      │
//!      │ Understand processor state
//!      ▼
//! 4. Cpu registers
//!      │
//!      │ Understand PC, SP, A, X, Y
//!      ▼
//! 5. fetch_byte / fetch_word
//!      │
//!      │ Understand instruction fetching
//!      ▼
//! 6. addressing modes
//!      │
//!      │ Understand operand/address calculation
//!      ▼
//! 7. execute()
//!      │
//!      │ Understand opcode decoding
//!      ▼
//! 8. individual instructions
//!      │
//!      │ Understand actual CPU behavior
//!      ▼
//! 9. cycle accounting
//!      │
//!      │ Understand timing
//!      ▼
//! 10. stack / BRK / RTI / JSR / RTS
//!         │
//!         └── Understand control flow and interrupts
//! ```
//!
//! ## Source-level documentation
//!
//! The implementation in `cpu.rs` intentionally follows the conceptual
//! organization of a 6502:
//!
//! ```text
//! StatusFlags
//!     │
//!     ├── processor status representation
//!     │
//! Cpu
//!     │
//!     ├── registers
//!     ├── reset
//!     ├── memory access
//!     ├── stack operations
//!     ├── opcode definitions
//!     ├── instruction implementations
//!     ├── addressing modes
//!     └── cycle accounting
//! ```
//!
//! The implementation can therefore be read from the hardware concepts
//! introduced in this file directly into the corresponding Rust functions.
//!
//! [`cpu`]: crate::cpu
//! [`mem`]: crate::mem
//! [`Cpu`]: crate::Cpu
//! [`Mem`]: crate::Mem
//! [`StatusFlags`]: crate::StatusFlags
//! [`StatusFlags::to_byte`]: crate::cpu::StatusFlags::to_byte
//! [`StatusFlags::from_byte`]: crate::cpu::StatusFlags::from_byte
//! [`Cpu::execute`]: crate::cpu::Cpu::execute
//! [`Cpu::fetch_word`]: crate::cpu::Cpu::fetch_word
//! [`Cpu::read_word`]: crate::cpu::Cpu::read_word
//! [`Cpu::sp_to_address`]: crate::cpu::Cpu::sp_to_address
//! [`Cpu::load_prg`]: crate::cpu::Cpu::load_prg
//! [`Cpu::reset`]: crate::cpu::Cpu::reset
//! [`Cpu::reset_at`]: crate::cpu::Cpu::reset_at
//! [`Mem::initialise`]: crate::mem::Mem::initialise
//! [`Index`]: std::ops::Index
//! [`IndexMut`]: std::ops::IndexMut
//! [`JSR`]: crate::cpu::Cpu::INS_JSR
//! [`RTS`]: crate::cpu::Cpu::INS_RTS

pub mod cpu;
pub mod mem;

pub use cpu::{Cpu, StatusFlags};
pub use mem::Mem;
