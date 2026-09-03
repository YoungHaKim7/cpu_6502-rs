use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

fn expect_unaffected_registers(cpu: &Cpu, cpu_before: &Cpu) {
    assert_eq!(cpu_before.flags.c, cpu.flags.c);
    assert_eq!(cpu_before.flags.i, cpu.flags.i);
    assert_eq!(cpu_before.flags.d, cpu.flags.d);
    assert_eq!(cpu_before.flags.b, cpu.flags.b);
    assert_eq!(cpu_before.flags.v, cpu.flags.v);
}

#[test]
fn tax_can_transfer_a_non_negative_non_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.a = 0x42;
    cpu.x = 0x32;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_TAX;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.x, 0x42);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn tax_can_transfer_a_non_negative_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.a = 0x0;
    cpu.x = 0x32;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_TAX;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.x, 0x0);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn tax_can_transfer_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.a = 0b1000_1011;
    cpu.x = 0x32;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_TAX;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0b1000_1011);
    assert_eq!(cpu.x, 0b1000_1011);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn tay_can_transfer_a_non_negative_non_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.a = 0x42;
    cpu.y = 0x32;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_TAY;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.y, 0x42);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn tay_can_transfer_a_non_negative_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.a = 0x0;
    cpu.y = 0x32;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_TAY;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.y, 0x0);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn tay_can_transfer_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.a = 0b1000_1011;
    cpu.y = 0x32;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_TAY;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0b1000_1011);
    assert_eq!(cpu.y, 0b1000_1011);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn txa_can_transfer_a_non_negative_non_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0x42;
    cpu.a = 0x32;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_TXA;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0x42);
    assert_eq!(cpu.a, 0x42);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn txa_can_transfer_a_non_negative_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0x0;
    cpu.a = 0x32;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_TXA;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0x0);
    assert_eq!(cpu.a, 0x0);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn txa_can_transfer_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0b1000_1011;
    cpu.a = 0x32;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_TXA;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0b1000_1011);
    assert_eq!(cpu.a, 0b1000_1011);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn tya_can_transfer_a_non_negative_non_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.y = 0x42;
    cpu.a = 0x32;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_TYA;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.y, 0x42);
    assert_eq!(cpu.a, 0x42);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn tya_can_transfer_a_non_negative_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.y = 0x0;
    cpu.a = 0x32;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_TYA;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.y, 0x0);
    assert_eq!(cpu.a, 0x0);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn tya_can_transfer_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.y = 0b1000_1011;
    cpu.a = 0x32;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_TYA;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.y, 0b1000_1011);
    assert_eq!(cpu.a, 0b1000_1011);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_registers(&cpu, &cpu_copy);
}
