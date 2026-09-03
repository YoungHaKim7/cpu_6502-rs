use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

#[test]
fn asl_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.a = 1;
    mem[0xFF00] = Cpu::INS_ASL;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 2);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn asl_can_shift_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = true;
    cpu.flags.n = false;
    cpu.a = 0b1100_0010;
    mem[0xFF00] = Cpu::INS_ASL;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0b1000_0100);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn asl_zero_page_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_ASL_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 1;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 2);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn asl_zero_page_can_shift_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ASL_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 0b1100_0010;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0b1000_0100);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn asl_zero_page_x_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ASL_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 1;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 2);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn asl_zero_page_x_can_shift_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = true;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ASL_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 0b1100_0010;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0b1000_0100);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn asl_abs_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_ASL_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 1;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 2);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn asl_abs_can_shift_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ASL_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0b1100_0010;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0b1000_0100);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn asl_abs_x_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ASL_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 1;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 2);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn asl_abs_x_can_shift_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = true;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ASL_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 0b1100_0010;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0b1000_0100);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn lsr_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.a = 1;
    mem[0xFF00] = Cpu::INS_LSR;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn lsr_can_shift_a_zero_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.a = 8;
    mem[0xFF00] = Cpu::INS_LSR;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 4);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn lsr_zero_page_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_LSR_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 1;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn lsr_zero_page_can_shift_a_zero_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_LSR_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 8;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 4);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn lsr_zero_page_x_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_LSR_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 1;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn lsr_zero_page_x_can_shift_a_zero_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_LSR_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 8;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 4);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn lsr_abs_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_LSR_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 1;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn lsr_abs_can_shift_a_zero_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_LSR_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 8;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 4);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn lsr_abs_x_can_shift_the_value_of_one() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_LSR_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 1;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn lsr_abs_x_can_shift_a_zero_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_LSR_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 8;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 4);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

// ------------ ROL ----------------

#[test]
fn rol_can_shift_a_bit_out_of_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.a = 0;
    mem[0xFF00] = Cpu::INS_ROL;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 1);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_can_shift_a_bit_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.a = 0b1000_0000;
    mem[0xFF00] = Cpu::INS_ROL;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_can_shift_zero_with_no_carry() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.a = 0;
    mem[0xFF00] = Cpu::INS_ROL;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0);
    assert!(!cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_can_shift_a_value_that_result_in_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.a = 0b0111_0011;
    mem[0xFF00] = Cpu::INS_ROL;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0b1110_0111);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

// ---------- Zero Page -------------

#[test]
fn rol_zero_page_can_shift_a_bit_out_of_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_ROL_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 0;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 1);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_zero_page_can_shift_a_bit_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_ROL_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 0b1000_0000;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_zero_page_can_shift_zero_with_no_carry() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_ROL_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 0;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0);
    assert!(!cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_zero_page_can_shift_a_value_that_result_in_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ROL_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 0b0111_0011;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0b1110_0111);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

// ------------- Zero Page X --------------

#[test]
fn rol_zero_page_x_can_shift_a_bit_out_of_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROL_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 0;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 1);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_zero_page_x_can_shift_a_bit_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROL_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 0b1000_0000;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_zero_page_x_can_shift_zero_with_no_carry() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROL_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 0;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0);
    assert!(!cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_zero_page_x_can_shift_a_value_that_result_in_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROL_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 0b0111_0011;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0b1110_0111);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

// ------------- Absolute --------------

#[test]
fn rol_absolute_can_shift_a_bit_out_of_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_ROL_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 1);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_absolute_can_shift_a_bit_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_ROL_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0b1000_0000;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_absolute_can_shift_zero_with_no_carry() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_ROL_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0);
    assert!(!cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_absolute_can_shift_a_value_that_result_in_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ROL_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0b0111_0011;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0b1110_0111);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

// ------------- Absolute X --------------

#[test]
fn rol_absolute_x_can_shift_a_bit_out_of_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROL_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 0;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 1);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_absolute_x_can_shift_a_bit_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROL_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 0b1000_0000;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_absolute_x_can_shift_zero_with_no_carry() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROL_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 0;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0);
    assert!(!cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn rol_absolute_x_can_shift_a_value_that_result_in_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROL_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 0b0111_0011;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0b1110_0111);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

// --------------- ROR --------------------

#[test]
fn ror_can_shift_the_carry_flag_into_the_operand() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.a = 0;
    mem[0xFF00] = Cpu::INS_ROR;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0b1000_0000);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn ror_can_shift_a_value_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.a = 1;
    mem[0xFF00] = Cpu::INS_ROR;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn ror_can_rotate_a_number() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = false;
    cpu.a = 0b0110_1101;
    mem[0xFF00] = Cpu::INS_ROR;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0b1011_0110);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

// ZeroPage

#[test]
fn ror_zero_page_can_shift_the_carry_flag_into_the_operand() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ROR_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 0;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0b1000_0000);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn ror_zero_page_can_shift_a_value_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ROR_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 1;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn ror_zero_page_can_rotate_a_number() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ROR_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 0b0110_1101;
    const EXPECTED_CYCLES: i32 = 5;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0b1011_0110);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

// Zero Page X

#[test]
fn ror_zero_x_page_can_shift_the_carry_flag_into_the_operand() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROR_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 0;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0b1000_0000);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn ror_zero_x_page_can_shift_a_value_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROR_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 1;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn ror_zero_x_page_can_rotate_a_number() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROR_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 0b0110_1101;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0b1011_0110);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

// Absolute

#[test]
fn ror_absolute_page_can_shift_the_carry_flag_into_the_operand() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ROR_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0b1000_0000);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn ror_absolute_page_can_shift_a_value_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ROR_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 1;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn ror_absolute_page_can_rotate_a_number() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_ROR_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0b0110_1101;
    const EXPECTED_CYCLES: i32 = 6;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0b1011_0110);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

// Absolute X

#[test]
fn ror_absolute_x_page_can_shift_the_carry_flag_into_the_operand() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROR_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 0;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0b1000_0000);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn ror_absolute_x_page_can_shift_a_value_into_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROR_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 1;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn ror_absolute_x_page_can_rotate_a_number() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    cpu.flags.z = true;
    cpu.flags.n = false;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_ROR_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 0b0110_1101;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0b1011_0110);
    assert!(cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}
