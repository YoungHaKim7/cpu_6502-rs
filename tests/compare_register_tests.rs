use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

fn expect_unaffected_registers(cpu: &Cpu, cpu_before: &Cpu) {
    assert_eq!(cpu_before.flags.i, cpu.flags.i);
    assert_eq!(cpu_before.flags.d, cpu.flags.d);
    assert_eq!(cpu_before.flags.b, cpu.flags.b);
    assert_eq!(cpu_before.flags.v, cpu.flags.v);
}

struct CmpTestData {
    register_value: u8,
    operand: u8,
    expect_c: bool,
    expect_z: bool,
    expect_n: bool,
}

fn compare_two_identical_values() -> CmpTestData {
    CmpTestData {
        register_value: 26,
        operand: 26,
        expect_z: true,
        expect_n: false,
        expect_c: true,
    }
}

fn compare_a_large_positive_to_a_small_positive() -> CmpTestData {
    CmpTestData {
        register_value: 48,
        operand: 26,
        expect_z: false,
        expect_n: false,
        expect_c: true,
    }
}

fn compare_a_negative_number_to_a_positive() -> CmpTestData {
    CmpTestData {
        register_value: 130, //Negative number!
        operand: 26,
        expect_z: false,
        expect_n: false,
        expect_c: true,
    }
}

fn compare_two_values_that_result_in_a_negative_flag_set() -> CmpTestData {
    CmpTestData {
        register_value: 8,
        operand: 26,
        expect_z: false,
        expect_n: true,
        expect_c: false,
    }
}

enum ERegister {
    A,
    X,
    Y,
}

fn compare_immediate(test: CmpTestData, register_to_compare: ERegister) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = !test.expect_c;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    let mut register: fn(&mut Cpu) -> &mut u8 = |cpu| &mut cpu.a;
    let mut opcode = Cpu::INS_CMP;
    match register_to_compare {
        ERegister::A => {}
        ERegister::X => {
            register = |cpu| &mut cpu.x;
            opcode = Cpu::INS_CPX;
        }
        ERegister::Y => {
            register = |cpu| &mut cpu.y;
            opcode = Cpu::INS_CPY;
        }
    }
    *register(&mut cpu) = test.register_value;

    mem[0xFF00] = opcode;
    mem[0xFF01] = test.operand;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(*register(&mut cpu), test.register_value);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.c, test.expect_c);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn compare_zero_page(test: CmpTestData, register_to_compare: ERegister) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = !test.expect_c;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;

    let mut register: fn(&mut Cpu) -> &mut u8 = |cpu| &mut cpu.a;
    let mut opcode = Cpu::INS_CMP_ZP;
    match register_to_compare {
        ERegister::A => {}
        ERegister::X => {
            register = |cpu| &mut cpu.x;
            opcode = Cpu::INS_CPX_ZP;
        }
        ERegister::Y => {
            register = |cpu| &mut cpu.y;
            opcode = Cpu::INS_CPY_ZP;
        }
    }
    *register(&mut cpu) = test.register_value;
    mem[0xFF00] = opcode;
    mem[0xFF01] = 0x42;
    mem[0x0042] = test.operand;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(*register(&mut cpu), test.register_value);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.c, test.expect_c);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn cmp_zero_page_x(test: CmpTestData) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = !test.expect_c;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.a = test.register_value;
    cpu.x = 4;
    mem[0xFF00] = Cpu::INS_CMP_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x4] = test.operand;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.register_value);
    assert_eq!(cpu.x, 4);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.c, test.expect_c);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn compare_absolute(test: CmpTestData, register_to_compare: ERegister) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = !test.expect_c;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;

    let mut register: fn(&mut Cpu) -> &mut u8 = |cpu| &mut cpu.a;
    let mut opcode = Cpu::INS_CMP_ABS;
    match register_to_compare {
        ERegister::A => {}
        ERegister::X => {
            register = |cpu| &mut cpu.x;
            opcode = Cpu::INS_CPX_ABS;
        }
        ERegister::Y => {
            register = |cpu| &mut cpu.y;
            opcode = Cpu::INS_CPY_ABS;
        }
    }
    *register(&mut cpu) = test.register_value;

    mem[0xFF00] = opcode;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = test.operand;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(*register(&mut cpu), test.register_value);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.c, test.expect_c);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn cmp_absolute_x(test: CmpTestData) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = !test.expect_c;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.a = test.register_value;
    cpu.x = 4;
    mem[0xFF00] = Cpu::INS_CMP_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 4] = test.operand;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.register_value);
    assert_eq!(cpu.x, 4);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.c, test.expect_c);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn cmp_absolute_y(test: CmpTestData) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = !test.expect_c;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.a = test.register_value;
    cpu.y = 4;
    mem[0xFF00] = Cpu::INS_CMP_ABSY;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 4] = test.operand;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.register_value);
    assert_eq!(cpu.y, 4);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.c, test.expect_c);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn cmp_indirect_x(test: CmpTestData) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = !test.expect_c;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.a = test.register_value;
    cpu.x = 4;
    mem[0xFF00] = Cpu::INS_CMP_INDX;
    mem[0xFF01] = 0x42;
    mem[0x42 + 4] = 0x00;
    mem[0x42 + 5] = 0x80;
    mem[0x8000] = test.operand;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.register_value);
    assert_eq!(cpu.x, 4);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.c, test.expect_c);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn cmp_indirect_y(test: CmpTestData) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = !test.expect_c;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.a = test.register_value;
    cpu.y = 4;
    mem[0xFF00] = Cpu::INS_CMP_INDY;
    mem[0xFF01] = 0x42;
    mem[0x42] = 0x00;
    mem[0x43] = 0x80;
    mem[0x8000 + 4] = test.operand;
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.register_value);
    assert_eq!(cpu.y, 4);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.c, test.expect_c);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

//-- Immediate

#[test]
fn cmp_immediate_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    compare_immediate(test, ERegister::A);
}

#[test]
fn cmp_immediate_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    compare_immediate(test, ERegister::A);
}

#[test]
fn cmp_immediate_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    compare_immediate(test, ERegister::A);
}

#[test]
fn cmp_immediate_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    compare_immediate(test, ERegister::A);
}

//-- Zero Page

#[test]
fn cmp_zero_page_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    compare_zero_page(test, ERegister::A);
}

#[test]
fn cmp_zero_page_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    compare_zero_page(test, ERegister::A);
}

#[test]
fn cmp_zero_page_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    compare_zero_page(test, ERegister::A);
}

#[test]
fn cmp_zero_page_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    compare_zero_page(test, ERegister::A);
}

//-- Zero Page X

#[test]
fn cmp_zero_page_x_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    cmp_zero_page_x(test);
}

#[test]
fn cmp_zero_page_x_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    cmp_zero_page_x(test);
}

#[test]
fn cmp_zero_page_x_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    cmp_zero_page_x(test);
}

#[test]
fn cmp_zero_page_x_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    cmp_zero_page_x(test);
}

//-- Absolute

#[test]
fn cmp_absolute_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    compare_absolute(test, ERegister::A);
}

#[test]
fn cmp_absolute_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    compare_absolute(test, ERegister::A);
}

#[test]
fn cmp_absolute_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    compare_absolute(test, ERegister::A);
}

#[test]
fn cmp_absolute_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    compare_absolute(test, ERegister::A);
}

//-- Absolute X

#[test]
fn cmp_absolute_x_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    cmp_absolute_x(test);
}

#[test]
fn cmp_absolute_x_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    cmp_absolute_x(test);
}

#[test]
fn cmp_absolute_x_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    cmp_absolute_x(test);
}

#[test]
fn cmp_absolute_x_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    cmp_absolute_x(test);
}

//-- Absolute Y

#[test]
fn cmp_absolute_y_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    cmp_absolute_y(test);
}

#[test]
fn cmp_absolute_y_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    cmp_absolute_y(test);
}

#[test]
fn cmp_absolute_y_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    cmp_absolute_y(test);
}

#[test]
fn cmp_absolute_y_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    cmp_absolute_y(test);
}

//-- Indirect X

#[test]
fn cmp_indirect_x_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    cmp_indirect_x(test);
}

#[test]
fn cmp_indirect_x_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    cmp_indirect_x(test);
}

#[test]
fn cmp_indirect_x_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    cmp_indirect_x(test);
}

#[test]
fn cmp_indirect_x_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    cmp_indirect_x(test);
}

//-- Indirect Y

#[test]
fn cmp_indirect_y_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    cmp_indirect_y(test);
}

#[test]
fn cmp_indirect_y_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    cmp_indirect_y(test);
}

#[test]
fn cmp_indirect_y_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    cmp_indirect_y(test);
}

#[test]
fn cmp_indirect_y_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    cmp_indirect_y(test);
}

//-- CPX Immediate

#[test]
fn cpx_immediate_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    compare_immediate(test, ERegister::X);
}

#[test]
fn cpx_immediate_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    compare_immediate(test, ERegister::X);
}

#[test]
fn cpx_immediate_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    compare_immediate(test, ERegister::X);
}

#[test]
fn cpx_immediate_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    compare_immediate(test, ERegister::X);
}

//-- CPY Immediate

#[test]
fn cpy_immediate_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    compare_immediate(test, ERegister::Y);
}

#[test]
fn cpy_immediate_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    compare_immediate(test, ERegister::Y);
}

#[test]
fn cpy_immediate_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    compare_immediate(test, ERegister::Y);
}

#[test]
fn cpy_immediate_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    compare_immediate(test, ERegister::Y);
}

//-- CPX Zero Page

#[test]
fn cpx_zero_page_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    compare_zero_page(test, ERegister::X);
}

#[test]
fn cpx_zero_page_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    compare_zero_page(test, ERegister::X);
}

#[test]
fn cpx_zero_page_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    compare_zero_page(test, ERegister::X);
}

#[test]
fn cpx_zero_page_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    compare_zero_page(test, ERegister::X);
}

//-- CPY Zero Page

#[test]
fn cpy_zero_page_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    compare_zero_page(test, ERegister::Y);
}

#[test]
fn cpy_zero_page_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    compare_zero_page(test, ERegister::Y);
}

#[test]
fn cpy_zero_page_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    compare_zero_page(test, ERegister::Y);
}

#[test]
fn cpy_zero_page_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    compare_zero_page(test, ERegister::Y);
}

//-- CPX Absolute

#[test]
fn cpx_absolute_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    compare_absolute(test, ERegister::X);
}

#[test]
fn cpx_absolute_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    compare_absolute(test, ERegister::X);
}

#[test]
fn cpx_absolute_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    compare_absolute(test, ERegister::X);
}

#[test]
fn cpx_absolute_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    compare_absolute(test, ERegister::X);
}

//-- CPY Absolute

#[test]
fn cpy_absolute_can_compare_two_identical_values() {
    let test = compare_two_identical_values();
    compare_absolute(test, ERegister::Y);
}

#[test]
fn cpy_absolute_can_compare_a_large_positive_to_a_small_positive() {
    let test = compare_a_large_positive_to_a_small_positive();
    compare_absolute(test, ERegister::Y);
}

#[test]
fn cpy_absolute_can_compare_a_negative_number_to_a_positive() {
    let test = compare_a_negative_number_to_a_positive();
    compare_absolute(test, ERegister::Y);
}

#[test]
fn cpy_absolute_can_compare_two_values_that_result_in_a_negative_flag_set() {
    let test = compare_two_values_that_result_in_a_negative_flag_set();
    compare_absolute(test, ERegister::Y);
}

// NOTE: the C++ source has one further test, `LoopTest`, disabled behind
// `#if 0`; its Rust equivalent would be:
//
// #[test]
// fn loop_test() {
//     // given:
//     /*
//      * = $1000
//
//      lda #0
//      clc
//      loop
//         adc #8
//         cmp #24
//         bne loop
//
//      ldx #20
//     */
//     let test_prg = [
//         0x0, 0x10, 0xA9, 0x00, 0x18, 0x69, 0x08, 0xC9, 0x18, 0xD0, 0xFA, 0xA2, 0x14,
//     ];
//
//     // when:
//     let start_address = cpu.load_prg(&test_prg, &mut mem);
//     cpu.pc = start_address;
//
//     // then:
//     let mut clock = 1000;
//     while clock > 0 {
//         clock -= cpu.execute(1, &mut mem);
//     }
// }
