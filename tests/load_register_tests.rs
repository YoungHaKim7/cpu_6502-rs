use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

fn verify_unmodified_flags_from_load_register(cpu: &Cpu, cpu_copy: &Cpu) {
    assert_eq!(cpu.flags.c, cpu_copy.flags.c);
    assert_eq!(cpu.flags.i, cpu_copy.flags.i);
    assert_eq!(cpu.flags.d, cpu_copy.flags.d);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.v, cpu_copy.flags.v);
}

/// (Port of the C++ fixture methods that took a `Byte CPU::*` member pointer;
/// the closure picks the register to test, e.g. `|cpu| &mut cpu.a`)
fn test_load_register_immediate(
    opcode_to_test: u8,
    register_to_test: impl Fn(&mut Cpu) -> &mut u8,
) {
    // given:
    let (mut cpu, mut mem) = setup();
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x84;

    //when:
    let cpu_copy = cpu;
    let cycles_used = cpu.execute(2, &mut mem);

    //then:
    assert_eq!(*register_to_test(&mut cpu), 0x84);
    assert_eq!(cycles_used, 2);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

fn test_load_register_zero_page(
    opcode_to_test: u8,
    register_to_test: impl Fn(&mut Cpu) -> &mut u8,
) {
    // given:
    let (mut cpu, mut mem) = setup();
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x42;
    mem[0x0042] = 0x37;

    //when:
    let cpu_copy = cpu;
    let cycles_used = cpu.execute(3, &mut mem);

    //then:
    assert_eq!(*register_to_test(&mut cpu), 0x37);
    assert_eq!(cycles_used, 3);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

fn test_load_register_zero_page_x(
    opcode_to_test: u8,
    register_to_test: impl Fn(&mut Cpu) -> &mut u8,
) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.x = 5;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x42;
    mem[0x0047] = 0x37;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(4, &mut mem);

    //then:
    assert_eq!(*register_to_test(&mut cpu), 0x37);
    assert_eq!(cycles_used, 4);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

fn test_load_register_zero_page_y(
    opcode_to_test: u8,
    register_to_test: impl Fn(&mut Cpu) -> &mut u8,
) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.y = 5;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x42;
    mem[0x0047] = 0x37;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(4, &mut mem);

    //then:
    assert_eq!(*register_to_test(&mut cpu), 0x37);
    assert_eq!(cycles_used, 4);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

fn test_load_register_absolute(
    opcode_to_test: u8,
    register_to_test: impl Fn(&mut Cpu) -> &mut u8,
) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0xFFFE] = 0x44; //0x4480
    mem[0x4480] = 0x37;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(*register_to_test(&mut cpu), 0x37);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

fn test_load_register_absolute_x(
    opcode_to_test: u8,
    register_to_test: impl Fn(&mut Cpu) -> &mut u8,
) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 1;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0xFFFE] = 0x44; //0x4480
    mem[0x4481] = 0x37;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(*register_to_test(&mut cpu), 0x37);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

fn test_load_register_absolute_y(
    opcode_to_test: u8,
    register_to_test: impl Fn(&mut Cpu) -> &mut u8,
) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.y = 1;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0xFFFE] = 0x44; //0x4480
    mem[0x4481] = 0x37;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(*register_to_test(&mut cpu), 0x37);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

fn test_load_register_absolute_x_when_crossing_page(
    opcode_to_test: u8,
    register_to_test: impl Fn(&mut Cpu) -> &mut u8,
) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.x = 0x1;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0xFF;
    mem[0xFFFE] = 0x44; //0x44FF
    mem[0x4500] = 0x37; //0x44FF+0x1 crosses page boundary!
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(*register_to_test(&mut cpu), 0x37);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

fn test_load_register_absolute_y_when_crossing_page(
    opcode_to_test: u8,
    register_to_test: impl Fn(&mut Cpu) -> &mut u8,
) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.y = 0x1;
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0xFF;
    mem[0xFFFE] = 0x44; //0x44FF
    mem[0x4500] = 0x37; //0x44FF+0x1 crosses page boundary!
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(*register_to_test(&mut cpu), 0x37);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

#[test]
fn the_cpu_does_nothing_when_we_execute_zero_cycles() {
    //given:
    let (mut cpu, mut mem) = setup();
    const NUM_CYCLES: i32 = 0;

    //when:
    let cycles_used = cpu.execute(NUM_CYCLES, &mut mem);

    //then:
    assert_eq!(cycles_used, 0);
}

#[test]
fn cpu_can_execute_more_cycles_than_requested_if_required_by_the_instruction() {
    // given:
    let (mut cpu, mut mem) = setup();
    mem[0xFFFC] = Cpu::INS_LDA_IM;
    mem[0xFFFD] = 0x84;
    let _cpu_copy = cpu;
    const NUM_CYCLES: i32 = 1;

    //when:
    let cycles_used = cpu.execute(NUM_CYCLES, &mut mem);

    //then:
    assert_eq!(cycles_used, 2);
}

#[test]
fn lda_immediate_can_load_a_value_into_the_a_register() {
    test_load_register_immediate(Cpu::INS_LDA_IM, |cpu| &mut cpu.a);
}

#[test]
fn ldx_immediate_can_load_a_value_into_the_x_register() {
    test_load_register_immediate(Cpu::INS_LDX_IM, |cpu| &mut cpu.x);
}

#[test]
fn ldy_immediate_can_load_a_value_into_the_y_register() {
    test_load_register_immediate(Cpu::INS_LDY_IM, |cpu| &mut cpu.y);
}

#[test]
fn lda_zero_page_can_load_a_value_into_the_a_register() {
    test_load_register_zero_page(Cpu::INS_LDA_ZP, |cpu| &mut cpu.a);
}

#[test]
fn ldx_zero_page_can_load_a_value_into_the_x_register() {
    test_load_register_zero_page(Cpu::INS_LDX_ZP, |cpu| &mut cpu.x);
}

#[test]
fn ldy_zero_page_can_load_a_value_into_the_y_register() {
    test_load_register_zero_page(Cpu::INS_LDY_ZP, |cpu| &mut cpu.y);
}

#[test]
fn lda_immediate_can_affect_the_zero_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0x44;
    mem[0xFFFC] = Cpu::INS_LDA_IM;
    mem[0xFFFD] = 0x0;
    let cpu_copy = cpu;

    //when:
    cpu.execute(2, &mut mem);

    //then:
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

#[test]
fn lda_zero_page_x_can_load_a_value_into_the_a_register() {
    test_load_register_zero_page_x(Cpu::INS_LDA_ZPX, |cpu| &mut cpu.a);
}

#[test]
fn ldx_zero_page_y_can_load_a_value_into_the_x_register() {
    test_load_register_zero_page_y(Cpu::INS_LDX_ZPY, |cpu| &mut cpu.x);
}

#[test]
fn ldy_zero_page_x_can_load_a_value_into_the_y_register() {
    test_load_register_zero_page_x(Cpu::INS_LDY_ZPX, |cpu| &mut cpu.y);
}

#[test]
fn lda_zero_page_x_can_load_a_value_into_the_a_register_when_it_wraps() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.x = 0xFF;
    mem[0xFFFC] = Cpu::INS_LDA_ZPX;
    mem[0xFFFD] = 0x80;
    mem[0x007F] = 0x37;

    //when:
    let cpu_copy = cpu;
    let cycles_used = cpu.execute(4, &mut mem);

    //then:
    assert_eq!(cpu.a, 0x37);
    assert_eq!(cycles_used, 4);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

#[test]
fn lda_absolute_can_load_a_value_into_the_a_register() {
    test_load_register_absolute(Cpu::INS_LDA_ABS, |cpu| &mut cpu.a);
}

#[test]
fn ldx_absolute_can_load_a_value_into_the_x_register() {
    test_load_register_absolute(Cpu::INS_LDX_ABS, |cpu| &mut cpu.x);
}

#[test]
fn ldy_absolute_can_load_a_value_into_the_y_register() {
    test_load_register_absolute(Cpu::INS_LDY_ABS, |cpu| &mut cpu.y);
}

#[test]
fn lda_absolute_x_can_load_a_value_into_the_a_register() {
    test_load_register_absolute_x(Cpu::INS_LDA_ABSX, |cpu| &mut cpu.a);
}

#[test]
fn ldx_absolute_y_can_load_a_value_into_the_x_register() {
    test_load_register_absolute_y(Cpu::INS_LDX_ABSY, |cpu| &mut cpu.x);
}

#[test]
fn ldy_absolute_x_can_load_a_value_into_the_y_register() {
    test_load_register_absolute_x(Cpu::INS_LDY_ABSX, |cpu| &mut cpu.y);
}

#[test]
fn lda_absolute_x_can_load_a_value_into_the_a_register_when_it_crosses_a_page_boundary() {
    test_load_register_absolute_x_when_crossing_page(Cpu::INS_LDA_ABSX, |cpu| &mut cpu.a);
}

#[test]
fn ldy_absolute_x_can_load_a_value_into_the_y_register_when_it_crosses_a_page_boundary() {
    test_load_register_absolute_x_when_crossing_page(Cpu::INS_LDY_ABSX, |cpu| &mut cpu.y);
}

#[test]
fn lda_absolute_y_can_load_a_value_into_the_a_register() {
    test_load_register_absolute_y(Cpu::INS_LDA_ABSY, |cpu| &mut cpu.a);
}

#[test]
fn lda_absolute_y_can_load_a_value_into_the_a_register_when_it_crosses_a_page_boundary() {
    test_load_register_absolute_y_when_crossing_page(Cpu::INS_LDA_ABSY, |cpu| &mut cpu.a);
}

#[test]
fn ldx_absolute_y_can_load_a_value_into_the_x_register_when_it_crosses_a_page_boundary() {
    test_load_register_absolute_y_when_crossing_page(Cpu::INS_LDX_ABSY, |cpu| &mut cpu.x);
}

#[test]
fn lda_indirect_x_can_load_a_value_into_the_a_register() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x04;
    mem[0xFFFC] = Cpu::INS_LDA_INDX;
    mem[0xFFFD] = 0x02;
    mem[0x0006] = 0x00; //0x2 + 0x4
    mem[0x0007] = 0x80;
    mem[0x8000] = 0x37;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(cpu.a, 0x37);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

#[test]
fn lda_indirect_y_can_load_a_value_into_the_a_register() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.y = 0x04;
    mem[0xFFFC] = Cpu::INS_LDA_INDY;
    mem[0xFFFD] = 0x02;
    mem[0x0002] = 0x00;
    mem[0x0003] = 0x80;
    mem[0x8004] = 0x37; //0x8000 + 0x4
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(cpu.a, 0x37);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}

#[test]
fn lda_indirect_y_can_load_a_value_into_the_a_register_when_it_crosses_a_page() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.y = 0x1;
    mem[0xFFFC] = Cpu::INS_LDA_INDY;
    mem[0xFFFD] = 0x05;
    mem[0x0005] = 0xFF;
    mem[0x0006] = 0x80;
    mem[0x8100] = 0x37; //0x80FF + 0x1
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(cpu.a, 0x37);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_load_register(&cpu, &cpu_copy);
}
