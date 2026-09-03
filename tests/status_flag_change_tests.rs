use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

#[test]
fn clc_will_clear_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    mem[0xFF00] = Cpu::INS_CLC;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert!(!cpu.flags.c);
    assert_eq!(cpu.flags.z, cpu_copy.flags.z);
    assert_eq!(cpu.flags.i, cpu_copy.flags.i);
    assert_eq!(cpu.flags.d, cpu_copy.flags.d);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.v, cpu_copy.flags.v);
    assert_eq!(cpu.flags.n, cpu_copy.flags.n);
}

#[test]
fn sec_will_set_the_carry_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    mem[0xFF00] = Cpu::INS_SEC;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert!(cpu.flags.c);
    assert_eq!(cpu.flags.z, cpu_copy.flags.z);
    assert_eq!(cpu.flags.i, cpu_copy.flags.i);
    assert_eq!(cpu.flags.d, cpu_copy.flags.d);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.v, cpu_copy.flags.v);
    assert_eq!(cpu.flags.n, cpu_copy.flags.n);
}

#[test]
fn cld_will_clear_the_decimal_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.d = true;
    mem[0xFF00] = Cpu::INS_CLD;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert!(!cpu.flags.d);
    assert_eq!(cpu.flags.z, cpu_copy.flags.z);
    assert_eq!(cpu.flags.i, cpu_copy.flags.i);
    assert_eq!(cpu.flags.c, cpu_copy.flags.c);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.v, cpu_copy.flags.v);
    assert_eq!(cpu.flags.n, cpu_copy.flags.n);
}

#[test]
fn sed_will_set_the_decimal_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.d = false;
    mem[0xFF00] = Cpu::INS_SED;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert!(cpu.flags.d);
    assert_eq!(cpu.flags.z, cpu_copy.flags.z);
    assert_eq!(cpu.flags.i, cpu_copy.flags.i);
    assert_eq!(cpu.flags.c, cpu_copy.flags.c);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.v, cpu_copy.flags.v);
    assert_eq!(cpu.flags.n, cpu_copy.flags.n);
}

#[test]
fn cli_will_clear_the_interrupt_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.i = true;
    mem[0xFF00] = Cpu::INS_CLI;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert!(!cpu.flags.i);
    assert_eq!(cpu.flags.z, cpu_copy.flags.z);
    assert_eq!(cpu.flags.d, cpu_copy.flags.d);
    assert_eq!(cpu.flags.c, cpu_copy.flags.c);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.v, cpu_copy.flags.v);
    assert_eq!(cpu.flags.n, cpu_copy.flags.n);
}

#[test]
fn sei_will_set_the_interrupt_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.i = false;
    mem[0xFF00] = Cpu::INS_SEI;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert!(cpu.flags.i);
    assert_eq!(cpu.flags.z, cpu_copy.flags.z);
    assert_eq!(cpu.flags.d, cpu_copy.flags.d);
    assert_eq!(cpu.flags.c, cpu_copy.flags.c);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.v, cpu_copy.flags.v);
    assert_eq!(cpu.flags.n, cpu_copy.flags.n);
}

#[test]
fn clv_will_clear_the_overflow_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.v = true;
    mem[0xFF00] = Cpu::INS_CLV;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert!(!cpu.flags.v);
    assert_eq!(cpu.flags.z, cpu_copy.flags.z);
    assert_eq!(cpu.flags.d, cpu_copy.flags.d);
    assert_eq!(cpu.flags.c, cpu_copy.flags.c);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.i, cpu_copy.flags.i);
    assert_eq!(cpu.flags.n, cpu_copy.flags.n);
}
