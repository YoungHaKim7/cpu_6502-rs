use crate::mem::Mem;

/// Processor status flags (port of `m6502::StatusFlags` bitfield).
///
/// The C++ version unions this bitfield with a `PS` byte; here the bools are
/// the single source of truth and [`StatusFlags::to_byte`] /
/// [`StatusFlags::from_byte`] do the bit packing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StatusFlags {
    pub c: bool,      //0: Carry Flag
    pub z: bool,      //1: Zero Flag
    pub i: bool,      //2: Interrupt disable
    pub d: bool,      //3: Decimal mode
    pub b: bool,      //4: Break
    pub unused: bool, //5: Unused
    pub v: bool,      //6: Overflow
    pub n: bool,      //7: Negative
}

impl StatusFlags {
    /// Pack the flags into the processor status byte `PS`
    pub fn to_byte(self) -> u8 {
        (self.c as u8)
            | ((self.z as u8) << 1)
            | ((self.i as u8) << 2)
            | ((self.d as u8) << 3)
            | ((self.b as u8) << 4)
            | ((self.unused as u8) << 5)
            | ((self.v as u8) << 6)
            | ((self.n as u8) << 7)
    }

    /// Unpack a processor status byte `PS` into the flags
    pub fn from_byte(byte: u8) -> Self {
        StatusFlags {
            c: byte & 0b0000_0001 != 0,
            z: byte & 0b0000_0010 != 0,
            i: byte & 0b0000_0100 != 0,
            d: byte & 0b0000_1000 != 0,
            b: byte & 0b0001_0000 != 0,
            unused: byte & 0b0010_0000 != 0,
            v: byte & 0b0100_0000 != 0,
            n: byte & 0b1000_0000 != 0,
        }
    }
}

/// The 6502 CPU (port of `m6502::CPU`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cpu {
    pub pc: u16, //program counter
    pub sp: u8,  //stack pointer

    pub a: u8,
    pub x: u8,
    pub y: u8, //registers

    pub flags: StatusFlags, //processor status
}

/// Which register an instruction operates on.
/// (Replaces the C++ `Byte& Register` member references)
#[derive(Clone, Copy)]
enum Reg {
    A,
    X,
    Y,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self, memory: &mut Mem) {
        self.reset_at(0xFFFC, memory);
    }

    pub fn reset_at(&mut self, reset_vector: u16, memory: &mut Mem) {
        self.pc = reset_vector;
        self.sp = 0xFF;
        self.flags = StatusFlags::default();
        self.a = 0;
        self.x = 0;
        self.y = 0;
        memory.initialise();
    }

    /// The processor status as a byte (the C++ `PS` union member)
    pub fn ps(&self) -> u8 {
        self.flags.to_byte()
    }

    fn fetch_byte(&mut self, cycles: &mut i32, memory: &Mem) -> u8 {
        let data = memory[self.pc];
        self.pc = self.pc.wrapping_add(1);
        *cycles -= 1;
        data
    }

    fn fetch_sbyte(&mut self, cycles: &mut i32, memory: &Mem) -> i8 {
        self.fetch_byte(cycles, memory) as i8
    }

    fn fetch_word(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        // 6502 is little endian
        let mut data = memory[self.pc] as u16;
        self.pc = self.pc.wrapping_add(1);

        data |= (memory[self.pc] as u16) << 8;
        self.pc = self.pc.wrapping_add(1);

        *cycles -= 2;
        data
    }

    fn read_byte(&self, cycles: &mut i32, address: u16, memory: &Mem) -> u8 {
        let data = memory[address];
        *cycles -= 1;
        data
    }

    fn read_word(&self, cycles: &mut i32, address: u16, memory: &Mem) -> u16 {
        let lo_byte = self.read_byte(cycles, address, memory) as u16;
        let hi_byte = self.read_byte(cycles, address.wrapping_add(1), memory) as u16;
        lo_byte | (hi_byte << 8)
    }

    /// write 1 byte to memory
    fn write_byte(&self, value: u8, cycles: &mut i32, address: u16, memory: &mut Mem) {
        memory[address] = value;
        *cycles -= 1;
    }

    /// write 2 bytes to memory
    /// (unused, like the C++ original - kept as part of the public memory API)
    pub fn write_word(&self, value: u16, cycles: &mut i32, address: u16, memory: &mut Mem) {
        memory[address] = (value & 0xFF) as u8;
        memory[address.wrapping_add(1)] = (value >> 8) as u8;
        *cycles -= 2;
    }

    /// @return the stack pointer as a full 16-bit address (in the 1st page)
    pub fn sp_to_address(&self) -> u16 {
        0x100 | self.sp as u16
    }

    fn push_word_to_stack(&mut self, cycles: &mut i32, memory: &mut Mem, value: u16) {
        self.write_byte((value >> 8) as u8, cycles, self.sp_to_address(), memory);
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte((value & 0xFF) as u8, cycles, self.sp_to_address(), memory);
        self.sp = self.sp.wrapping_sub(1);
    }

    /// Push the PC-1 onto the stack
    fn push_pc_minus_one_to_stack(&mut self, cycles: &mut i32, memory: &mut Mem) {
        let pc = self.pc;
        self.push_word_to_stack(cycles, memory, pc.wrapping_sub(1));
    }

    /// Push the PC+1 onto the stack
    fn push_pc_plus_one_to_stack(&mut self, cycles: &mut i32, memory: &mut Mem) {
        let pc = self.pc;
        self.push_word_to_stack(cycles, memory, pc.wrapping_add(1));
    }

    fn push_byte_onto_stack(&mut self, cycles: &mut i32, value: u8, memory: &mut Mem) {
        let sp_word = self.sp_to_address();
        memory[sp_word] = value;
        *cycles -= 1;
        self.sp = self.sp.wrapping_sub(1);
        *cycles -= 1;
    }

    fn pop_byte_from_stack(&mut self, cycles: &mut i32, memory: &Mem) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        *cycles -= 1;
        let sp_word = self.sp_to_address();
        let value = memory[sp_word];
        *cycles -= 1;
        value
    }

    /// Pop a 16-bit value from the stack
    fn pop_word_from_stack(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let value_from_stack = self.read_word(cycles, self.sp_to_address().wrapping_add(1), memory);
        self.sp = self.sp.wrapping_add(2);
        *cycles -= 1;
        value_from_stack
    }

    // Process status bits
    pub const NEGATIVE_FLAG_BIT: u8 = 0b1000_0000;
    pub const OVERFLOW_FLAG_BIT: u8 = 0b0100_0000;
    pub const BREAK_FLAG_BIT: u8 = 0b0001_0000;
    pub const UNUSED_FLAG_BIT: u8 = 0b0010_0000;
    pub const INTERRUPT_DISABLE_FLAG_BIT: u8 = 0b0000_0100;
    pub const ZERO_BIT: u8 = 0b0000_0001;

    // opcodes
    //LDA
    pub const INS_LDA_IM: u8 = 0xA9;
    pub const INS_LDA_ZP: u8 = 0xA5;
    pub const INS_LDA_ZPX: u8 = 0xB5;
    pub const INS_LDA_ABS: u8 = 0xAD;
    pub const INS_LDA_ABSX: u8 = 0xBD;
    pub const INS_LDA_ABSY: u8 = 0xB9;
    pub const INS_LDA_INDX: u8 = 0xA1;
    pub const INS_LDA_INDY: u8 = 0xB1;
    //LDX
    pub const INS_LDX_IM: u8 = 0xA2;
    pub const INS_LDX_ZP: u8 = 0xA6;
    pub const INS_LDX_ZPY: u8 = 0xB6;
    pub const INS_LDX_ABS: u8 = 0xAE;
    pub const INS_LDX_ABSY: u8 = 0xBE;
    //LDY
    pub const INS_LDY_IM: u8 = 0xA0;
    pub const INS_LDY_ZP: u8 = 0xA4;
    pub const INS_LDY_ZPX: u8 = 0xB4;
    pub const INS_LDY_ABS: u8 = 0xAC;
    pub const INS_LDY_ABSX: u8 = 0xBC;
    //STA
    pub const INS_STA_ZP: u8 = 0x85;
    pub const INS_STA_ZPX: u8 = 0x95;
    pub const INS_STA_ABS: u8 = 0x8D;
    pub const INS_STA_ABSX: u8 = 0x9D;
    pub const INS_STA_ABSY: u8 = 0x99;
    pub const INS_STA_INDX: u8 = 0x81;
    pub const INS_STA_INDY: u8 = 0x91;
    //STX
    pub const INS_STX_ZP: u8 = 0x86;
    pub const INS_STX_ZPY: u8 = 0x96;
    pub const INS_STX_ABS: u8 = 0x8E;
    //STY
    pub const INS_STY_ZP: u8 = 0x84;
    pub const INS_STY_ZPX: u8 = 0x94;
    pub const INS_STY_ABS: u8 = 0x8C;

    pub const INS_TSX: u8 = 0xBA;
    pub const INS_TXS: u8 = 0x9A;
    pub const INS_PHA: u8 = 0x48;
    pub const INS_PLA: u8 = 0x68;
    pub const INS_PHP: u8 = 0x08;
    pub const INS_PLP: u8 = 0x28;

    pub const INS_JMP_ABS: u8 = 0x4C;
    pub const INS_JMP_IND: u8 = 0x6C;
    pub const INS_JSR: u8 = 0x20;
    pub const INS_RTS: u8 = 0x60;

    //Logical Ops

    //AND
    pub const INS_AND_IM: u8 = 0x29;
    pub const INS_AND_ZP: u8 = 0x25;
    pub const INS_AND_ZPX: u8 = 0x35;
    pub const INS_AND_ABS: u8 = 0x2D;
    pub const INS_AND_ABSX: u8 = 0x3D;
    pub const INS_AND_ABSY: u8 = 0x39;
    pub const INS_AND_INDX: u8 = 0x21;
    pub const INS_AND_INDY: u8 = 0x31;

    //OR
    pub const INS_ORA_IM: u8 = 0x09;
    pub const INS_ORA_ZP: u8 = 0x05;
    pub const INS_ORA_ZPX: u8 = 0x15;
    pub const INS_ORA_ABS: u8 = 0x0D;
    pub const INS_ORA_ABSX: u8 = 0x1D;
    pub const INS_ORA_ABSY: u8 = 0x19;
    pub const INS_ORA_INDX: u8 = 0x01;
    pub const INS_ORA_INDY: u8 = 0x11;

    //EOR
    pub const INS_EOR_IM: u8 = 0x49;
    pub const INS_EOR_ZP: u8 = 0x45;
    pub const INS_EOR_ZPX: u8 = 0x55;
    pub const INS_EOR_ABS: u8 = 0x4D;
    pub const INS_EOR_ABSX: u8 = 0x5D;
    pub const INS_EOR_ABSY: u8 = 0x59;
    pub const INS_EOR_INDX: u8 = 0x41;
    pub const INS_EOR_INDY: u8 = 0x51;

    //BIT
    pub const INS_BIT_ZP: u8 = 0x24;
    pub const INS_BIT_ABS: u8 = 0x2C;

    //Transfer Registers
    pub const INS_TAX: u8 = 0xAA;
    pub const INS_TAY: u8 = 0xA8;
    pub const INS_TXA: u8 = 0x8A;
    pub const INS_TYA: u8 = 0x98;

    //Increments, Decrements
    pub const INS_INX: u8 = 0xE8;
    pub const INS_INY: u8 = 0xC8;
    pub const INS_DEY: u8 = 0x88;
    pub const INS_DEX: u8 = 0xCA;
    pub const INS_DEC_ZP: u8 = 0xC6;
    pub const INS_DEC_ZPX: u8 = 0xD6;
    pub const INS_DEC_ABS: u8 = 0xCE;
    pub const INS_DEC_ABSX: u8 = 0xDE;
    pub const INS_INC_ZP: u8 = 0xE6;
    pub const INS_INC_ZPX: u8 = 0xF6;
    pub const INS_INC_ABS: u8 = 0xEE;
    pub const INS_INC_ABSX: u8 = 0xFE;

    //branches
    pub const INS_BEQ: u8 = 0xF0;
    pub const INS_BNE: u8 = 0xD0;
    pub const INS_BCS: u8 = 0xB0;
    pub const INS_BCC: u8 = 0x90;
    pub const INS_BMI: u8 = 0x30;
    pub const INS_BPL: u8 = 0x10;
    pub const INS_BVC: u8 = 0x50;
    pub const INS_BVS: u8 = 0x70;

    //status flag changes
    pub const INS_CLC: u8 = 0x18;
    pub const INS_SEC: u8 = 0x38;
    pub const INS_CLD: u8 = 0xD8;
    pub const INS_SED: u8 = 0xF8;
    pub const INS_CLI: u8 = 0x58;
    pub const INS_SEI: u8 = 0x78;
    pub const INS_CLV: u8 = 0xB8;

    //Arithmetic
    pub const INS_ADC: u8 = 0x69;
    pub const INS_ADC_ZP: u8 = 0x65;
    pub const INS_ADC_ZPX: u8 = 0x75;
    pub const INS_ADC_ABS: u8 = 0x6D;
    pub const INS_ADC_ABSX: u8 = 0x7D;
    pub const INS_ADC_ABSY: u8 = 0x79;
    pub const INS_ADC_INDX: u8 = 0x61;
    pub const INS_ADC_INDY: u8 = 0x71;

    pub const INS_SBC: u8 = 0xE9;
    pub const INS_SBC_ABS: u8 = 0xED;
    pub const INS_SBC_ZP: u8 = 0xE5;
    pub const INS_SBC_ZPX: u8 = 0xF5;
    pub const INS_SBC_ABSX: u8 = 0xFD;
    pub const INS_SBC_ABSY: u8 = 0xF9;
    pub const INS_SBC_INDX: u8 = 0xE1;
    pub const INS_SBC_INDY: u8 = 0xF1;

    // Register Comparison
    pub const INS_CMP: u8 = 0xC9;
    pub const INS_CMP_ZP: u8 = 0xC5;
    pub const INS_CMP_ZPX: u8 = 0xD5;
    pub const INS_CMP_ABS: u8 = 0xCD;
    pub const INS_CMP_ABSX: u8 = 0xDD;
    pub const INS_CMP_ABSY: u8 = 0xD9;
    pub const INS_CMP_INDX: u8 = 0xC1;
    pub const INS_CMP_INDY: u8 = 0xD1;

    pub const INS_CPX: u8 = 0xE0;
    pub const INS_CPY: u8 = 0xC0;
    pub const INS_CPX_ZP: u8 = 0xE4;
    pub const INS_CPY_ZP: u8 = 0xC4;
    pub const INS_CPX_ABS: u8 = 0xEC;
    pub const INS_CPY_ABS: u8 = 0xCC;

    // shifts
    pub const INS_ASL: u8 = 0x0A;
    pub const INS_ASL_ZP: u8 = 0x06;
    pub const INS_ASL_ZPX: u8 = 0x16;
    pub const INS_ASL_ABS: u8 = 0x0E;
    pub const INS_ASL_ABSX: u8 = 0x1E;

    pub const INS_LSR: u8 = 0x4A;
    pub const INS_LSR_ZP: u8 = 0x46;
    pub const INS_LSR_ZPX: u8 = 0x56;
    pub const INS_LSR_ABS: u8 = 0x4E;
    pub const INS_LSR_ABSX: u8 = 0x5E;

    pub const INS_ROL: u8 = 0x2A;
    pub const INS_ROL_ZP: u8 = 0x26;
    pub const INS_ROL_ZPX: u8 = 0x36;
    pub const INS_ROL_ABS: u8 = 0x2E;
    pub const INS_ROL_ABSX: u8 = 0x3E;

    pub const INS_ROR: u8 = 0x6A;
    pub const INS_ROR_ZP: u8 = 0x66;
    pub const INS_ROR_ZPX: u8 = 0x76;
    pub const INS_ROR_ABS: u8 = 0x6E;
    pub const INS_ROR_ABSX: u8 = 0x7E;

    //misc
    pub const INS_NOP: u8 = 0xEA;
    pub const INS_BRK: u8 = 0x00;
    pub const INS_RTI: u8 = 0x40;

    /// Sets the correct Process status after a load register instruction
    /// - LDA, LDX, LDY
    fn set_zero_and_negative_flags(&mut self, register: u8) {
        self.flags.z = register == 0;
        self.flags.n = register & Self::NEGATIVE_FLAG_BIT > 0;
    }

    /** Load a Register with the value from the memory address */
    fn load_register(&mut self, cycles: &mut i32, address: u16, reg: Reg, memory: &Mem) {
        let value = self.read_byte(cycles, address, memory);
        match reg {
            Reg::A => self.a = value,
            Reg::X => self.x = value,
            Reg::Y => self.y = value,
        }
        self.set_zero_and_negative_flags(value);
    }

    /** And the A Register with the value from the memory address */
    fn and(&mut self, cycles: &mut i32, address: u16, memory: &Mem) {
        self.a &= self.read_byte(cycles, address, memory);
        self.set_zero_and_negative_flags(self.a);
    }

    /** Or the A Register with the value from the memory address */
    fn ora(&mut self, cycles: &mut i32, address: u16, memory: &Mem) {
        self.a |= self.read_byte(cycles, address, memory);
        self.set_zero_and_negative_flags(self.a);
    }

    /** Eor the A Register with the value from the memory address */
    fn eor(&mut self, cycles: &mut i32, address: u16, memory: &Mem) {
        self.a ^= self.read_byte(cycles, address, memory);
        self.set_zero_and_negative_flags(self.a);
    }

    /* Conditional branch */
    fn branch_if(&mut self, cycles: &mut i32, test: bool, expected: bool, memory: &Mem) {
        let offset = self.fetch_sbyte(cycles, memory);
        if test == expected {
            let pc_old = self.pc;
            self.pc = self.pc.wrapping_add_signed(offset as i16);
            *cycles -= 1;

            let page_changed = (self.pc >> 8) != (pc_old >> 8);
            if page_changed {
                *cycles -= 1;
            }
        }
    }

    /** Do add with carry given the the operand */
    fn adc(&mut self, operand: u8) {
        assert!(!self.flags.d, "haven't handled decimal mode!");
        let are_sign_bits_the_same = (self.a ^ operand) & Self::NEGATIVE_FLAG_BIT == 0;
        let mut sum: u16 = self.a as u16;
        sum += operand as u16;
        sum += self.flags.c as u16;
        self.a = (sum & 0xFF) as u8;
        self.set_zero_and_negative_flags(self.a);
        self.flags.c = sum > 0xFF;
        self.flags.v = are_sign_bits_the_same && (self.a ^ operand) & Self::NEGATIVE_FLAG_BIT != 0;
    }

    /** Do subtract with carry given the the operand */
    fn sbc(&mut self, operand: u8) {
        self.adc(!operand);
    }

    /** Sets the processor status for a CMP/CPX/CPY instruction */
    fn register_compare(&mut self, operand: u8, register_value: u8) {
        let temp = register_value.wrapping_sub(operand);
        self.flags.n = temp & Self::NEGATIVE_FLAG_BIT > 0;
        self.flags.z = register_value == operand;
        self.flags.c = register_value >= operand;
    }

    /** Arithmetic shift left */
    fn asl(&mut self, cycles: &mut i32, operand: u8) -> u8 {
        self.flags.c = operand & Self::NEGATIVE_FLAG_BIT > 0;
        let result = operand << 1;
        self.set_zero_and_negative_flags(result);
        *cycles -= 1;
        result
    }

    /** Logical shift right */
    fn lsr(&mut self, cycles: &mut i32, operand: u8) -> u8 {
        self.flags.c = operand & Self::ZERO_BIT > 0;
        let result = operand >> 1;
        self.set_zero_and_negative_flags(result);
        *cycles -= 1;
        result
    }

    /** Rotate left */
    fn rol(&mut self, cycles: &mut i32, operand: u8) -> u8 {
        let new_bit0 = if self.flags.c { Self::ZERO_BIT } else { 0 };
        self.flags.c = operand & Self::NEGATIVE_FLAG_BIT > 0;
        let mut result = operand << 1;
        result |= new_bit0;
        self.set_zero_and_negative_flags(result);
        *cycles -= 1;
        result
    }

    /** Rotate right */
    fn ror(&mut self, cycles: &mut i32, operand: u8) -> u8 {
        let old_bit0 = operand & Self::ZERO_BIT > 0;
        let mut result = operand >> 1;
        if self.flags.c {
            result |= Self::NEGATIVE_FLAG_BIT;
        }
        *cycles -= 1;
        self.flags.c = old_bit0;
        self.set_zero_and_negative_flags(result);
        result
    }

    /** Push Processor status onto the stack
     *    Setting bits 4 & 5 on the stack */
    fn push_ps_to_stack(&mut self, cycles: &mut i32, memory: &mut Mem) {
        let ps_stack = self.ps() | Self::BREAK_FLAG_BIT | Self::UNUSED_FLAG_BIT;
        self.push_byte_onto_stack(cycles, ps_stack, memory);
    }

    /** Pop Processor status from the stack
     *    Clearing bits 4 & 5 (Break & Unused) */
    fn pop_ps_from_stack(&mut self, cycles: &mut i32, memory: &Mem) {
        let ps = self.pop_byte_from_stack(cycles, memory);
        self.flags = StatusFlags::from_byte(ps);
        self.flags.b = false;
        self.flags.unused = false;
    }

    /** @return the address that the program was loading into, or 0 if no program */
    pub fn load_prg(&self, program: &[u8], memory: &mut Mem) -> u16 {
        let mut load_address: u16 = 0;
        if program.len() > 2 {
            let lo = program[0] as u16;
            let hi = (program[1] as u16) << 8;
            load_address = lo | hi;
            let mut at = load_address;
            for &byte in &program[2..] {
                memory[at] = byte;
                at = at.wrapping_add(1);
            }
        }

        load_address
    }

    /** printf the registers, program counter etc */
    pub fn print_status(&self) {
        println!("A: {} X: {} Y: {}", self.a, self.x, self.y);
        println!("PC: {} SP: {}", self.pc, self.sp);
        println!("PS: {}", self.ps());
    }

    /** @return the number of cycles that were used */
    pub fn execute(&mut self, mut cycles: i32, memory: &mut Mem) -> i32 {
        let cycles_requested = cycles;
        while cycles > 0 {
            let ins = self.fetch_byte(&mut cycles, memory);
            match ins {
                Self::INS_AND_IM => {
                    self.a &= self.fetch_byte(&mut cycles, memory);
                    self.set_zero_and_negative_flags(self.a);
                }
                Self::INS_ORA_IM => {
                    self.a |= self.fetch_byte(&mut cycles, memory);
                    self.set_zero_and_negative_flags(self.a);
                }
                Self::INS_EOR_IM => {
                    self.a ^= self.fetch_byte(&mut cycles, memory);
                    self.set_zero_and_negative_flags(self.a);
                }
                Self::INS_AND_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    self.and(&mut cycles, address, memory);
                }
                Self::INS_ORA_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    self.ora(&mut cycles, address, memory);
                }
                Self::INS_EOR_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    self.eor(&mut cycles, address, memory);
                }
                Self::INS_AND_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    self.and(&mut cycles, address, memory);
                }
                Self::INS_ORA_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    self.ora(&mut cycles, address, memory);
                }
                Self::INS_EOR_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    self.eor(&mut cycles, address, memory);
                }
                Self::INS_AND_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.and(&mut cycles, address, memory);
                }
                Self::INS_ORA_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.ora(&mut cycles, address, memory);
                }
                Self::INS_EOR_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.eor(&mut cycles, address, memory);
                }
                Self::INS_AND_ABSX => {
                    let address = self.addr_absolute_x(&mut cycles, memory);
                    self.and(&mut cycles, address, memory);
                }
                Self::INS_ORA_ABSX => {
                    let address = self.addr_absolute_x(&mut cycles, memory);
                    self.ora(&mut cycles, address, memory);
                }
                Self::INS_EOR_ABSX => {
                    let address = self.addr_absolute_x(&mut cycles, memory);
                    self.eor(&mut cycles, address, memory);
                }
                Self::INS_AND_ABSY => {
                    let address = self.addr_absolute_y(&mut cycles, memory);
                    self.and(&mut cycles, address, memory);
                }
                Self::INS_ORA_ABSY => {
                    let address = self.addr_absolute_y(&mut cycles, memory);
                    self.ora(&mut cycles, address, memory);
                }
                Self::INS_EOR_ABSY => {
                    let address = self.addr_absolute_y(&mut cycles, memory);
                    self.eor(&mut cycles, address, memory);
                }
                Self::INS_AND_INDX => {
                    let address = self.addr_indirect_x(&mut cycles, memory);
                    self.and(&mut cycles, address, memory);
                }
                Self::INS_ORA_INDX => {
                    let address = self.addr_indirect_x(&mut cycles, memory);
                    self.ora(&mut cycles, address, memory);
                }
                Self::INS_EOR_INDX => {
                    let address = self.addr_indirect_x(&mut cycles, memory);
                    self.eor(&mut cycles, address, memory);
                }
                Self::INS_AND_INDY => {
                    let address = self.addr_indirect_y(&mut cycles, memory);
                    self.and(&mut cycles, address, memory);
                }
                Self::INS_ORA_INDY => {
                    let address = self.addr_indirect_y(&mut cycles, memory);
                    self.ora(&mut cycles, address, memory);
                }
                Self::INS_EOR_INDY => {
                    let address = self.addr_indirect_y(&mut cycles, memory);
                    self.eor(&mut cycles, address, memory);
                }
                Self::INS_BIT_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, address, memory);
                    self.flags.z = self.a & value == 0;
                    self.flags.n = value & Self::NEGATIVE_FLAG_BIT != 0;
                    self.flags.v = value & Self::OVERFLOW_FLAG_BIT != 0;
                }
                Self::INS_BIT_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, address, memory);
                    self.flags.z = self.a & value == 0;
                    self.flags.n = value & Self::NEGATIVE_FLAG_BIT != 0;
                    self.flags.v = value & Self::OVERFLOW_FLAG_BIT != 0;
                }
                Self::INS_LDA_IM => {
                    self.a = self.fetch_byte(&mut cycles, memory);
                    self.set_zero_and_negative_flags(self.a);
                }
                Self::INS_LDX_IM => {
                    self.x = self.fetch_byte(&mut cycles, memory);
                    self.set_zero_and_negative_flags(self.x);
                }
                Self::INS_LDY_IM => {
                    self.y = self.fetch_byte(&mut cycles, memory);
                    self.set_zero_and_negative_flags(self.y);
                }
                Self::INS_LDA_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::A, memory);
                }
                Self::INS_LDX_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::X, memory);
                }
                Self::INS_LDX_ZPY => {
                    let address = self.addr_zero_page_y(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::X, memory);
                }
                Self::INS_LDY_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::Y, memory);
                }
                Self::INS_LDA_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::A, memory);
                }
                Self::INS_LDY_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::Y, memory);
                }
                Self::INS_LDA_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::A, memory);
                }
                Self::INS_LDX_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::X, memory);
                }
                Self::INS_LDY_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::Y, memory);
                }
                Self::INS_LDA_ABSX => {
                    let address = self.addr_absolute_x(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::A, memory);
                }
                Self::INS_LDY_ABSX => {
                    let address = self.addr_absolute_x(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::Y, memory);
                }
                Self::INS_LDA_ABSY => {
                    let address = self.addr_absolute_y(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::A, memory);
                }
                Self::INS_LDX_ABSY => {
                    let address = self.addr_absolute_y(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::X, memory);
                }
                Self::INS_LDA_INDX => {
                    let address = self.addr_indirect_x(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::A, memory);
                }
                Self::INS_STA_INDX => {
                    let address = self.addr_indirect_x(&mut cycles, memory);
                    self.write_byte(self.a, &mut cycles, address, memory);
                }
                Self::INS_LDA_INDY => {
                    let address = self.addr_indirect_y(&mut cycles, memory);
                    self.load_register(&mut cycles, address, Reg::A, memory);
                }
                Self::INS_STA_INDY => {
                    let address = self.addr_indirect_y_6(&mut cycles, memory);
                    self.write_byte(self.a, &mut cycles, address, memory);
                }
                Self::INS_STA_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    self.write_byte(self.a, &mut cycles, address, memory);
                }
                Self::INS_STX_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    self.write_byte(self.x, &mut cycles, address, memory);
                }
                Self::INS_STX_ZPY => {
                    let address = self.addr_zero_page_y(&mut cycles, memory);
                    self.write_byte(self.x, &mut cycles, address, memory);
                }
                Self::INS_STY_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    self.write_byte(self.y, &mut cycles, address, memory);
                }
                Self::INS_STA_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.write_byte(self.a, &mut cycles, address, memory);
                }
                Self::INS_STX_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.write_byte(self.x, &mut cycles, address, memory);
                }
                Self::INS_STY_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.write_byte(self.y, &mut cycles, address, memory);
                }
                Self::INS_STA_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    self.write_byte(self.a, &mut cycles, address, memory);
                }
                Self::INS_STY_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    self.write_byte(self.y, &mut cycles, address, memory);
                }
                Self::INS_STA_ABSX => {
                    let address = self.addr_absolute_x_5(&mut cycles, memory);
                    self.write_byte(self.a, &mut cycles, address, memory);
                }
                Self::INS_STA_ABSY => {
                    let address = self.addr_absolute_y_5(&mut cycles, memory);
                    self.write_byte(self.a, &mut cycles, address, memory);
                }
                Self::INS_JSR => {
                    let sub_addr = self.fetch_word(&mut cycles, memory);
                    self.push_pc_minus_one_to_stack(&mut cycles, memory);
                    self.pc = sub_addr;
                    cycles -= 1;
                }
                Self::INS_RTS => {
                    let return_address = self.pop_word_from_stack(&mut cycles, memory);
                    self.pc = return_address.wrapping_add(1);
                    cycles -= 2;
                }
                //TODO:
                //An original 6502 has does not correctly fetch the target
                //address if the indirect vector falls on a page boundary
                //( e.g.$xxFF where xx is any value from $00 to $FF ).
                //In this case fetches the LSB from $xxFF as expected but
                //takes the MSB from $xx00.This is fixed in some later chips
                //like the 65SC02 so for compatibility always ensure the
                //indirect vector is not at the end of the page.
                Self::INS_JMP_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    self.pc = address;
                }
                Self::INS_JMP_IND => {
                    let mut address = self.addr_absolute(&mut cycles, memory);
                    address = self.read_word(&mut cycles, address, memory);
                    self.pc = address;
                }
                Self::INS_TSX => {
                    self.x = self.sp;
                    cycles -= 1;
                    self.set_zero_and_negative_flags(self.x);
                }
                Self::INS_TXS => {
                    self.sp = self.x;
                    cycles -= 1;
                }
                Self::INS_PHA => {
                    self.push_byte_onto_stack(&mut cycles, self.a, memory);
                }
                Self::INS_PLA => {
                    self.a = self.pop_byte_from_stack(&mut cycles, memory);
                    self.set_zero_and_negative_flags(self.a);
                    cycles -= 1;
                }
                Self::INS_PHP => {
                    self.push_ps_to_stack(&mut cycles, memory);
                }
                Self::INS_PLP => {
                    self.pop_ps_from_stack(&mut cycles, memory);
                    cycles -= 1;
                }
                Self::INS_TAX => {
                    self.x = self.a;
                    cycles -= 1;
                    self.set_zero_and_negative_flags(self.x);
                }
                Self::INS_TAY => {
                    self.y = self.a;
                    cycles -= 1;
                    self.set_zero_and_negative_flags(self.y);
                }
                Self::INS_TXA => {
                    self.a = self.x;
                    cycles -= 1;
                    self.set_zero_and_negative_flags(self.a);
                }
                Self::INS_TYA => {
                    self.a = self.y;
                    cycles -= 1;
                    self.set_zero_and_negative_flags(self.a);
                }
                Self::INS_INX => {
                    self.x = self.x.wrapping_add(1);
                    cycles -= 1;
                    self.set_zero_and_negative_flags(self.x);
                }
                Self::INS_INY => {
                    self.y = self.y.wrapping_add(1);
                    cycles -= 1;
                    self.set_zero_and_negative_flags(self.y);
                }
                Self::INS_DEX => {
                    self.x = self.x.wrapping_sub(1);
                    cycles -= 1;
                    self.set_zero_and_negative_flags(self.x);
                }
                Self::INS_DEY => {
                    self.y = self.y.wrapping_sub(1);
                    cycles -= 1;
                    self.set_zero_and_negative_flags(self.y);
                }
                Self::INS_DEC_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let mut value = self.read_byte(&mut cycles, address, memory);
                    value = value.wrapping_sub(1);
                    cycles -= 1;
                    self.write_byte(value, &mut cycles, address, memory);
                    self.set_zero_and_negative_flags(value);
                }
                Self::INS_DEC_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    let mut value = self.read_byte(&mut cycles, address, memory);
                    value = value.wrapping_sub(1);
                    cycles -= 1;
                    self.write_byte(value, &mut cycles, address, memory);
                    self.set_zero_and_negative_flags(value);
                }
                Self::INS_DEC_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let mut value = self.read_byte(&mut cycles, address, memory);
                    value = value.wrapping_sub(1);
                    cycles -= 1;
                    self.write_byte(value, &mut cycles, address, memory);
                    self.set_zero_and_negative_flags(value);
                }
                Self::INS_DEC_ABSX => {
                    let address = self.addr_absolute_x_5(&mut cycles, memory);
                    let mut value = self.read_byte(&mut cycles, address, memory);
                    value = value.wrapping_sub(1);
                    cycles -= 1;
                    self.write_byte(value, &mut cycles, address, memory);
                    self.set_zero_and_negative_flags(value);
                }
                Self::INS_INC_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let mut value = self.read_byte(&mut cycles, address, memory);
                    value = value.wrapping_add(1);
                    cycles -= 1;
                    self.write_byte(value, &mut cycles, address, memory);
                    self.set_zero_and_negative_flags(value);
                }
                Self::INS_INC_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    let mut value = self.read_byte(&mut cycles, address, memory);
                    value = value.wrapping_add(1);
                    cycles -= 1;
                    self.write_byte(value, &mut cycles, address, memory);
                    self.set_zero_and_negative_flags(value);
                }
                Self::INS_INC_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let mut value = self.read_byte(&mut cycles, address, memory);
                    value = value.wrapping_add(1);
                    cycles -= 1;
                    self.write_byte(value, &mut cycles, address, memory);
                    self.set_zero_and_negative_flags(value);
                }
                Self::INS_INC_ABSX => {
                    let address = self.addr_absolute_x_5(&mut cycles, memory);
                    let mut value = self.read_byte(&mut cycles, address, memory);
                    value = value.wrapping_add(1);
                    cycles -= 1;
                    self.write_byte(value, &mut cycles, address, memory);
                    self.set_zero_and_negative_flags(value);
                }
                Self::INS_BEQ => {
                    self.branch_if(&mut cycles, self.flags.z, true, memory);
                }
                Self::INS_BNE => {
                    self.branch_if(&mut cycles, self.flags.z, false, memory);
                }
                Self::INS_BCS => {
                    self.branch_if(&mut cycles, self.flags.c, true, memory);
                }
                Self::INS_BCC => {
                    self.branch_if(&mut cycles, self.flags.c, false, memory);
                }
                Self::INS_BMI => {
                    self.branch_if(&mut cycles, self.flags.n, true, memory);
                }
                Self::INS_BPL => {
                    self.branch_if(&mut cycles, self.flags.n, false, memory);
                }
                Self::INS_BVC => {
                    self.branch_if(&mut cycles, self.flags.v, false, memory);
                }
                Self::INS_BVS => {
                    self.branch_if(&mut cycles, self.flags.v, true, memory);
                }
                Self::INS_CLC => {
                    self.flags.c = false;
                    cycles -= 1;
                }
                Self::INS_SEC => {
                    self.flags.c = true;
                    cycles -= 1;
                }
                Self::INS_CLD => {
                    self.flags.d = false;
                    cycles -= 1;
                }
                Self::INS_SED => {
                    self.flags.d = true;
                    cycles -= 1;
                }
                Self::INS_CLI => {
                    self.flags.i = false;
                    cycles -= 1;
                }
                Self::INS_SEI => {
                    self.flags.i = true;
                    cycles -= 1;
                }
                Self::INS_CLV => {
                    self.flags.v = false;
                    cycles -= 1;
                }
                Self::INS_NOP => {
                    cycles -= 1;
                }
                Self::INS_ADC_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.adc(operand);
                }
                Self::INS_ADC_ABSX => {
                    let address = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.adc(operand);
                }
                Self::INS_ADC_ABSY => {
                    let address = self.addr_absolute_y(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.adc(operand);
                }
                Self::INS_ADC_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.adc(operand);
                }
                Self::INS_ADC_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.adc(operand);
                }
                Self::INS_ADC_INDX => {
                    let address = self.addr_indirect_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.adc(operand);
                }
                Self::INS_ADC_INDY => {
                    let address = self.addr_indirect_y(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.adc(operand);
                }
                Self::INS_ADC => {
                    let operand = self.fetch_byte(&mut cycles, memory);
                    self.adc(operand);
                }
                Self::INS_SBC => {
                    let operand = self.fetch_byte(&mut cycles, memory);
                    self.sbc(operand);
                }
                Self::INS_SBC_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.sbc(operand);
                }
                Self::INS_SBC_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.sbc(operand);
                }
                Self::INS_SBC_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.sbc(operand);
                }
                Self::INS_SBC_ABSX => {
                    let address = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.sbc(operand);
                }
                Self::INS_SBC_ABSY => {
                    let address = self.addr_absolute_y(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.sbc(operand);
                }
                Self::INS_SBC_INDX => {
                    let address = self.addr_indirect_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.sbc(operand);
                }
                Self::INS_SBC_INDY => {
                    let address = self.addr_indirect_y(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.sbc(operand);
                }
                Self::INS_CPX => {
                    let operand = self.fetch_byte(&mut cycles, memory);
                    self.register_compare(operand, self.x);
                }
                Self::INS_CPY => {
                    let operand = self.fetch_byte(&mut cycles, memory);
                    self.register_compare(operand, self.y);
                }
                Self::INS_CPX_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.x);
                }
                Self::INS_CPY_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.y);
                }
                Self::INS_CPX_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.x);
                }
                Self::INS_CPY_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.y);
                }
                Self::INS_CMP => {
                    let operand = self.fetch_byte(&mut cycles, memory);
                    self.register_compare(operand, self.a);
                }
                Self::INS_CMP_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.a);
                }
                Self::INS_CMP_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.a);
                }
                Self::INS_CMP_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.a);
                }
                Self::INS_CMP_ABSX => {
                    let address = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.a);
                }
                Self::INS_CMP_ABSY => {
                    let address = self.addr_absolute_y(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.a);
                }
                Self::INS_CMP_INDX => {
                    let address = self.addr_indirect_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.a);
                }
                Self::INS_CMP_INDY => {
                    let address = self.addr_indirect_y(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    self.register_compare(operand, self.a);
                }
                Self::INS_ASL => {
                    self.a = self.asl(&mut cycles, self.a);
                }
                Self::INS_ASL_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.asl(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ASL_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.asl(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ASL_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.asl(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ASL_ABSX => {
                    let address = self.addr_absolute_x_5(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.asl(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_LSR => {
                    self.a = self.lsr(&mut cycles, self.a);
                }
                Self::INS_LSR_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.lsr(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_LSR_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.lsr(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_LSR_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.lsr(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_LSR_ABSX => {
                    let address = self.addr_absolute_x_5(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.lsr(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ROL => {
                    self.a = self.rol(&mut cycles, self.a);
                }
                Self::INS_ROL_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.rol(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ROL_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.rol(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ROL_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.rol(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ROL_ABSX => {
                    let address = self.addr_absolute_x_5(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.rol(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ROR => {
                    self.a = self.ror(&mut cycles, self.a);
                }
                Self::INS_ROR_ZP => {
                    let address = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.ror(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ROR_ZPX => {
                    let address = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.ror(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ROR_ABS => {
                    let address = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.ror(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_ROR_ABSX => {
                    let address = self.addr_absolute_x_5(&mut cycles, memory);
                    let operand = self.read_byte(&mut cycles, address, memory);
                    let result = self.ror(&mut cycles, operand);
                    self.write_byte(result, &mut cycles, address, memory);
                }
                Self::INS_BRK => {
                    self.push_pc_plus_one_to_stack(&mut cycles, memory);
                    self.push_ps_to_stack(&mut cycles, memory);
                    const INTERRUPT_VECTOR: u16 = 0xFFFE;
                    self.pc = self.read_word(&mut cycles, INTERRUPT_VECTOR, memory);
                    self.flags.b = true;
                    self.flags.i = true;
                }
                Self::INS_RTI => {
                    self.pop_ps_from_stack(&mut cycles, memory);
                    self.pc = self.pop_word_from_stack(&mut cycles, memory);
                }
                _ => {
                    panic!("Instruction {:#04X} not handled", ins);
                }
            }
        }

        cycles_requested - cycles
    }

    /** Addressing mode - Zero page */
    fn addr_zero_page(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        self.fetch_byte(cycles, memory) as u16
    }

    /** Addressing mode - Zero page with X offset */
    fn addr_zero_page_x(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let zero_page_addr = self.fetch_byte(cycles, memory);
        let zero_page_addr = zero_page_addr.wrapping_add(self.x);
        *cycles -= 1;
        zero_page_addr as u16
    }

    /** Addressing mode - Zero page with Y offset */
    fn addr_zero_page_y(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let zero_page_addr = self.fetch_byte(cycles, memory);
        let zero_page_addr = zero_page_addr.wrapping_add(self.y);
        *cycles -= 1;
        zero_page_addr as u16
    }

    /** Addressing mode - Absolute */
    fn addr_absolute(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        self.fetch_word(cycles, memory)
    }

    /** Addressing mode - Absolute with X offset */
    fn addr_absolute_x(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let abs_address = self.fetch_word(cycles, memory);
        let abs_address_x = abs_address.wrapping_add(self.x as u16);
        let crossed_page_boundary = (abs_address ^ abs_address_x) >> 8 != 0;
        if crossed_page_boundary {
            *cycles -= 1;
        }

        abs_address_x
    }

    /** Addressing mode - Absolute with X offset
     *    - Always takes a cycle for the X page boundary)
     *    - See "STA Absolute,X" */
    fn addr_absolute_x_5(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let abs_address = self.fetch_word(cycles, memory);
        let abs_address_x = abs_address.wrapping_add(self.x as u16);
        *cycles -= 1;
        abs_address_x
    }

    /** Addressing mode - Absolute with Y offset */
    fn addr_absolute_y(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let abs_address = self.fetch_word(cycles, memory);
        let abs_address_y = abs_address.wrapping_add(self.y as u16);
        let crossed_page_boundary = (abs_address ^ abs_address_y) >> 8 != 0;
        if crossed_page_boundary {
            *cycles -= 1;
        }

        abs_address_y
    }

    /** Addressing mode - Absolute with Y offset
     *    - Always takes a cycle for the Y page boundary)
     *    - See "STA Absolute,Y" */
    fn addr_absolute_y_5(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let abs_address = self.fetch_word(cycles, memory);
        let abs_address_y = abs_address.wrapping_add(self.y as u16);
        *cycles -= 1;
        abs_address_y
    }

    /** Addressing mode - Indirect X | Indexed Indirect */
    fn addr_indirect_x(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let zp_address = self.fetch_byte(cycles, memory);
        let zp_address = zp_address.wrapping_add(self.x);
        *cycles -= 1;
        self.read_word(cycles, zp_address as u16, memory)
    }

    /** Addressing mode - Indirect Y | Indirect Indexed */
    fn addr_indirect_y(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let zp_address = self.fetch_byte(cycles, memory);
        let effective_addr = self.read_word(cycles, zp_address as u16, memory);
        let effective_addr_y = effective_addr.wrapping_add(self.y as u16);
        let crossed_page_boundary = (effective_addr ^ effective_addr_y) >> 8 != 0;
        if crossed_page_boundary {
            *cycles -= 1;
        }
        effective_addr_y
    }

    /** Addressing mode - Indirect Y | Indirect Indexed
     *    - Always takes a cycle for the Y page boundary)
     *    - See "STA (Indirect,Y) */
    fn addr_indirect_y_6(&mut self, cycles: &mut i32, memory: &Mem) -> u16 {
        let zp_address = self.fetch_byte(cycles, memory);
        let effective_addr = self.read_word(cycles, zp_address as u16, memory);
        let effective_addr_y = effective_addr.wrapping_add(self.y as u16);
        *cycles -= 1;
        effective_addr_y
    }
}
