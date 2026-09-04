use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

fn verify_unmodified_flags_from_store_register(cpu: &Cpu, cpu_copy: &Cpu) {
    assert_eq!(cpu.flags.c, cpu_copy.flags.c);
    assert_eq!(cpu.flags.i, cpu_copy.flags.i);
    assert_eq!(cpu.flags.d, cpu_copy.flags.d);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.v, cpu_copy.flags.v);
    assert_eq!(cpu.flags.z, cpu_copy.flags.z);
    assert_eq!(cpu.flags.n, cpu_copy.flags.n);
}

fn test_store_register_zero_page(opcode_to_test: u8, register: impl Fn(&mut Cpu) -> &mut u8) {
    // given:
    let (mut cpu, mut mem) = setup();
    *register(&mut cpu) = 0x2F;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0x0080] = 0x00;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0080], 0x2F);
    verify_unmodified_flags_from_store_register(&cpu, &cpu_copy);
}

fn test_store_register_absolute(opcode_to_test: u8, register: impl Fn(&mut Cpu) -> &mut u8) {
    // given:
    let (mut cpu, mut mem) = setup();
    *register(&mut cpu) = 0x2F;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    mem[0x8000] = 0x00;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0x2F);
    verify_unmodified_flags_from_store_register(&cpu, &cpu_copy);
}

fn test_store_register_zero_page_x(opcode_to_test: u8, register: impl Fn(&mut Cpu) -> &mut u8) {
    // given:
    let (mut cpu, mut mem) = setup();
    *register(&mut cpu) = 0x42;
    cpu.x = 0x0F;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0x008F] = 0x00;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x008F], 0x42);
    verify_unmodified_flags_from_store_register(&cpu, &cpu_copy);
}

fn test_store_register_zero_page_y(opcode_to_test: u8, register: impl Fn(&mut Cpu) -> &mut u8) {
    // given:
    let (mut cpu, mut mem) = setup();
    *register(&mut cpu) = 0x42;
    cpu.y = 0x0F;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0x008F] = 0x00;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x008F], 0x42);
    verify_unmodified_flags_from_store_register(&cpu, &cpu_copy);
}

#[test]
fn sta_zero_page_can_store_the_a_register_into_memory() {
    test_store_register_zero_page(Cpu::INS_STA_ZP, |cpu| &mut cpu.a);
}

#[test]
fn stx_zero_page_can_store_the_x_register_into_memory() {
    test_store_register_zero_page(Cpu::INS_STX_ZP, |cpu| &mut cpu.x);
}

#[test]
fn stx_zero_page_y_can_store_the_x_register_into_memory() {
    test_store_register_zero_page_y(Cpu::INS_STX_ZPY, |cpu| &mut cpu.x);
}

#[test]
fn sty_zero_page_can_store_the_y_register_into_memory() {
    test_store_register_zero_page(Cpu::INS_STY_ZP, |cpu| &mut cpu.y);
}

#[test]
fn sta_absolute_can_store_the_a_register_into_memory() {
    test_store_register_absolute(Cpu::INS_STA_ABS, |cpu| &mut cpu.a);
}

#[test]
fn stx_absolute_can_store_the_x_register_into_memory() {
    test_store_register_absolute(Cpu::INS_STX_ABS, |cpu| &mut cpu.x);
}

#[test]
fn sty_absolute_can_store_the_y_register_into_memory() {
    test_store_register_absolute(Cpu::INS_STY_ABS, |cpu| &mut cpu.y);
}

#[test]
fn sta_zero_page_x_can_store_the_a_register_into_memory() {
    test_store_register_zero_page_x(Cpu::INS_STA_ZPX, |cpu| &mut cpu.a);
}

#[test]
fn sty_zero_page_x_can_store_the_y_register_into_memory() {
    test_store_register_zero_page_x(Cpu::INS_STY_ZPX, |cpu| &mut cpu.y);
}

#[test]
fn sta_absolute_x_can_store_the_a_register_into_memory() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0x42;
    cpu.x = 0x0F;
    mem[0xFFFC] = Cpu::INS_STA_ABSX;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x800F], 0x42);
    verify_unmodified_flags_from_store_register(&cpu, &cpu_copy);
}

#[test]
fn sta_absolute_y_can_store_the_a_register_into_memory() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0x42;
    cpu.y = 0x0F;
    mem[0xFFFC] = Cpu::INS_STA_ABSY;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x800F], 0x42);
    verify_unmodified_flags_from_store_register(&cpu, &cpu_copy);
}

#[test]
fn sta_indirect_x_can_store_the_a_register_into_memory() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0x42;
    cpu.x = 0x0F;
    mem[0xFFFC] = Cpu::INS_STA_INDX;
    mem[0xFFFD] = 0x20;
    mem[0x002F] = 0x00;
    mem[0x0030] = 0x80;
    mem[0x8000] = 0x00;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0x42);
    verify_unmodified_flags_from_store_register(&cpu, &cpu_copy);
}

#[test]
fn sta_indirect_y_can_store_the_a_register_into_memory() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0x42;
    cpu.y = 0x0F;
    mem[0xFFFC] = Cpu::INS_STA_INDY;
    mem[0xFFFD] = 0x20;
    mem[0x0020] = 0x00;
    mem[0x0021] = 0x80;
    mem[0x8000 + 0x0F] = 0x00;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x0F], 0x42);
    verify_unmodified_flags_from_store_register(&cpu, &cpu_copy);
}
