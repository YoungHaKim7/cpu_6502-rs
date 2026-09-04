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
}

struct AdcTestData {
    carry: bool,
    a: u8,
    operand: u8,
    answer: u8,

    expect_c: bool,
    expect_z: bool,
    expect_n: bool,
    expect_v: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EOperation {
    Add,
    Subtract,
}

fn test_sbc_absolute(test: AdcTestData) {
    test_adc_absolute(test, EOperation::Subtract);
}

fn test_adc_absolute(test: AdcTestData, operation: EOperation) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = test.carry;
    cpu.a = test.a;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.flags.v = !test.expect_v;
    mem[0xFF00] = if operation == EOperation::Add {
        Cpu::INS_ADC_ABS
    } else {
        Cpu::INS_SBC_ABS
    };
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = test.operand;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.answer);
    assert_eq!(cpu.flags.c, test.expect_c);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.v, test.expect_v);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn test_sbc_absolute_x(test: AdcTestData) {
    test_adc_absolute_x(test, EOperation::Subtract);
}

fn test_adc_absolute_x(test: AdcTestData, operation: EOperation) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = test.carry;
    cpu.x = 0x10;
    cpu.a = test.a;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.flags.v = !test.expect_v;
    mem[0xFF00] = if operation == EOperation::Add {
        Cpu::INS_ADC_ABSX
    } else {
        Cpu::INS_SBC_ABSX
    };
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = test.operand;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.answer);
    assert_eq!(cpu.flags.c, test.expect_c);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.v, test.expect_v);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn test_sbc_absolute_y(test: AdcTestData) {
    test_adc_absolute_y(test, EOperation::Subtract);
}

fn test_adc_absolute_y(test: AdcTestData, operation: EOperation) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = test.carry;
    cpu.y = 0x10;
    cpu.a = test.a;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.flags.v = !test.expect_v;
    mem[0xFF00] = if operation == EOperation::Add {
        Cpu::INS_ADC_ABSY
    } else {
        Cpu::INS_SBC_ABSY
    };
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = test.operand;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.answer);
    assert_eq!(cpu.flags.c, test.expect_c);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.v, test.expect_v);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn test_sbc_immediate(test: AdcTestData) {
    test_adc_immediate(test, EOperation::Subtract);
}

fn test_adc_immediate(test: AdcTestData, operation: EOperation) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = test.carry;
    cpu.a = test.a;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.flags.v = !test.expect_v;
    mem[0xFF00] = if operation == EOperation::Add {
        Cpu::INS_ADC
    } else {
        Cpu::INS_SBC
    };
    mem[0xFF01] = test.operand;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.answer);
    assert_eq!(cpu.flags.c, test.expect_c);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.v, test.expect_v);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn test_sbc_zero_page(test: AdcTestData) {
    test_adc_zero_page(test, EOperation::Subtract);
}

fn test_adc_zero_page(test: AdcTestData, operation: EOperation) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = test.carry;
    cpu.a = test.a;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.flags.v = !test.expect_v;
    mem[0xFF00] = if operation == EOperation::Add {
        Cpu::INS_ADC_ZP
    } else {
        Cpu::INS_SBC_ZP
    };
    mem[0xFF01] = 0x42;
    mem[0x0042] = test.operand;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.answer);
    assert_eq!(cpu.flags.c, test.expect_c);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.v, test.expect_v);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn test_sbc_zero_page_x(test: AdcTestData) {
    test_adc_zero_page_x(test, EOperation::Subtract);
}

fn test_adc_zero_page_x(test: AdcTestData, operation: EOperation) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = test.carry;
    cpu.x = 0x10;
    cpu.a = test.a;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.flags.v = !test.expect_v;
    mem[0xFF00] = if operation == EOperation::Add {
        Cpu::INS_ADC_ZPX
    } else {
        Cpu::INS_SBC_ZPX
    };
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = test.operand;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.answer);
    assert_eq!(cpu.flags.c, test.expect_c);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.v, test.expect_v);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn test_sbc_indirect_x(test: AdcTestData) {
    test_adc_indirect_x(test, EOperation::Subtract);
}

fn test_adc_indirect_x(test: AdcTestData, operation: EOperation) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = test.carry;
    cpu.x = 0x04;
    cpu.a = test.a;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.flags.v = !test.expect_v;
    mem[0xFF00] = if operation == EOperation::Add {
        Cpu::INS_ADC_INDX
    } else {
        Cpu::INS_SBC_INDX
    };
    mem[0xFF01] = 0x02;
    mem[0x0006] = 0x00; //0x2 + 0x4
    mem[0x0007] = 0x80;
    mem[0x8000] = test.operand;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.answer);
    assert_eq!(cpu.flags.c, test.expect_c);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.v, test.expect_v);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

fn test_sbc_indirect_y(test: AdcTestData) {
    test_adc_indirect_y(test, EOperation::Subtract);
}

fn test_adc_indirect_y(test: AdcTestData, operation: EOperation) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = test.carry;
    cpu.y = 0x04;
    cpu.a = test.a;
    cpu.flags.z = !test.expect_z;
    cpu.flags.n = !test.expect_n;
    cpu.flags.v = !test.expect_v;
    mem[0xFF00] = if operation == EOperation::Add {
        Cpu::INS_ADC_INDY
    } else {
        Cpu::INS_SBC_INDY
    };
    mem[0xFF01] = 0x02;
    mem[0x0002] = 0x00;
    mem[0x0003] = 0x80;
    mem[0x8000 + 0x04] = test.operand;
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, test.answer);
    assert_eq!(cpu.flags.c, test.expect_c);
    assert_eq!(cpu.flags.z, test.expect_z);
    assert_eq!(cpu.flags.n, test.expect_n);
    assert_eq!(cpu.flags.v, test.expect_v);
    expect_unaffected_registers(&cpu, &cpu_copy);
}

#[test]
fn adc_abs_can_add_zero_to_zero_and_get_zero() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 0,
        answer: 0,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_adc_absolute(test, EOperation::Add);
}

#[test]
fn adc_abs_can_add_carry_and_zero_to_zero_and_get_one() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 0,
        answer: 1,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_absolute(test, EOperation::Add);
}

#[test]
fn adc_abs_can_add_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 38,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_absolute(test, EOperation::Add);
}

#[test]
fn adc_abs_can_add_a_positive_and_negative_number() {
    // A: 00010100 20
    // O: 11101111 -17
    // =: 00000011
    // C:1 N:0 V:0 Z:0
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: (-17i32) as u8,
        answer: 4,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_absolute(test, EOperation::Add);
}

#[test]
fn adc_abs_can_add_one_to_ff_and_it_will_cause_a_carry() {
    let test = AdcTestData {
        carry: false,
        a: 0xFF,
        operand: 1,
        answer: 0,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_adc_absolute(test, EOperation::Add);
}

#[test]
fn adc_abs_will_set_the_negative_flag_when_the_result_is_negative() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: (-1i32) as u8,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_adc_absolute(test, EOperation::Add);
}

#[test]
fn adc_abs_will_set_the_overflow_flag_when_signed_negative_addtion_fails() {
    // A: 10000000 -128
    // O: 11111111 -1
    // =: 01111111 127
    // C:1 N:0 V:1 Z:0
    let test = AdcTestData {
        carry: false,
        a: (-128i32) as u8,
        operand: (-1i32) as u8,
        answer: 127,
        expect_c: true,
        expect_n: false,
        expect_v: true,
        expect_z: false,
    };
    test_adc_absolute(test, EOperation::Add);
}

#[test]
fn adc_abs_will_set_the_overflow_flag_when_signed_negative_addtion_passed_due_to_inital_carry_flag()
{
    // C: 00000001
    // A: 10000000 -128
    // O: 11111111 -1
    // =: 10000000 -128
    // C:1 N:1 V:0 Z:0
    let test = AdcTestData {
        carry: true,
        a: (-128i32) as u8,
        operand: (-1i32) as u8,
        answer: (-128i32) as u8,
        expect_c: true,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_adc_absolute(test, EOperation::Add);
}

#[test]
fn adc_abs_will_set_the_overflow_flag_when_signed_positive_addtion_fails() {
    // A: 01111111 127
    // O: 00000001 1
    // =: 10000000 128
    // C:0 N:1 V:1 Z:0
    let test = AdcTestData {
        carry: false,
        a: 127,
        operand: 1,
        answer: 128,
        expect_c: false,
        expect_n: true,
        expect_v: true,
        expect_z: false,
    };
    test_adc_absolute(test, EOperation::Add);
}

#[test]
fn adc_immediate_can_add_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 38,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_immediate(test, EOperation::Add);
}

#[test]
fn adc_immediate_can_add_a_positive_and_negative_number() {
    // A: 00010100 20
    // O: 11101111 -17
    // =: 00000011
    // C:1 N:0 V:0 Z:0
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: (-17i32) as u8,
        answer: 4,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_immediate(test, EOperation::Add);
}

#[test]
fn adc_zero_page_can_add_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 38,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_zero_page(test, EOperation::Add);
}

#[test]
fn adc_zero_page_can_add_a_positive_and_negative_number() {
    // A: 00010100 20
    // O: 11101111 -17
    // =: 00000011
    // C:1 N:0 V:0 Z:0
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: (-17i32) as u8,
        answer: 4,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_zero_page(test, EOperation::Add);
}

#[test]
fn adc_zero_page_x_can_add_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 38,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_zero_page_x(test, EOperation::Add);
}

#[test]
fn adc_zero_page_x_can_add_a_positive_and_negative_number() {
    // A: 00010100 20
    // O: 11101111 -17
    // =: 00000011
    // C:1 N:0 V:0 Z:0
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: (-17i32) as u8,
        answer: 4,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_zero_page_x(test, EOperation::Add);
}

#[test]
fn adc_abs_x_can_add_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 38,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_absolute_x(test, EOperation::Add);
}

#[test]
fn adc_abs_x_can_add_a_positive_and_negative_number() {
    // A: 00010100 20
    // O: 11101111 -17
    // =: 00000011
    // C:1 N:0 V:0 Z:0
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: (-17i32) as u8,
        answer: 4,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_absolute_x(test, EOperation::Add);
}

#[test]
fn adc_abs_y_can_add_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 38,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_absolute_y(test, EOperation::Add);
}

#[test]
fn adc_abs_y_can_add_a_positive_and_negative_number() {
    // A: 00010100 20
    // O: 11101111 -17
    // =: 00000011
    // C:1 N:0 V:0 Z:0
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: (-17i32) as u8,
        answer: 4,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_absolute_y(test, EOperation::Add);
}

#[test]
fn adc_ind_x_can_add_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 38,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_indirect_x(test, EOperation::Add);
}

#[test]
fn adc_ind_x_can_add_a_positive_and_negative_number() {
    // A: 00010100 20
    // O: 11101111 -17
    // =: 00000011
    // C:1 N:0 V:0 Z:0
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: (-17i32) as u8,
        answer: 4,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_indirect_x(test, EOperation::Add);
}

#[test]
fn adc_ind_y_can_add_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 38,
        expect_c: false,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_indirect_y(test, EOperation::Add);
}

#[test]
fn adc_ind_y_can_add_a_positive_and_negative_number() {
    // A: 00010100 20
    // O: 11101111 -17
    // =: 00000011
    // C:1 N:0 V:0 Z:0
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: (-17i32) as u8,
        answer: 4,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_adc_indirect_y(test, EOperation::Add);
}

// SBC Abs --------------

#[test]
fn sbc_abs_can_subtract_zero_from_zero_and_get_zero() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 0,
        answer: 0,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_sbc_absolute(test);
}

#[test]
fn sbc_abs_can_subtract_zero_from_zero_and_carry_and_get_minus_one() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 0,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute(test);
}

#[test]
fn sbc_abs_can_subtract_one_from_zero_and_get_minus_one() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 1,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute(test);
}

#[test]
fn sbc_abs_can_subtract_one_from_zero_with_carry_and_get_minus_two() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 1,
        answer: (-2i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute(test);
}

#[test]
fn sbc_abs_can_subtract_two_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: (-128i32) as u8,
        operand: 1,
        answer: 127,
        expect_c: true,
        expect_n: false,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_absolute(test);
}

#[test]
fn sbc_abs_can_subtract_a_postitive_and_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: 127,
        operand: (-1i32) as u8,
        answer: 128,
        expect_c: false,
        expect_n: true,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_absolute(test);
}

#[test]
fn sbc_abs_can_subtract_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 3,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute(test);
}

#[test]
fn sbc_abs_can_subtract_two_negative_numbers() {
    let test = AdcTestData {
        carry: true,
        a: (-20i32) as u8,
        operand: (-17i32) as u8,
        answer: (-3i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute(test);
}

// SBC Zero Page

#[test]
fn sbc_zero_page_can_subtract_zero_from_zero_and_get_zero() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 0,
        answer: 0,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_sbc_zero_page(test);
}

#[test]
fn sbc_zero_page_can_subtract_zero_from_zero_and_carry_and_get_minus_one() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 0,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page(test);
}

#[test]
fn sbc_zero_page_can_subtract_one_from_zero_and_get_minus_one() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 1,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page(test);
}

#[test]
fn sbc_zero_page_can_subtract_one_from_zero_with_carry_and_get_minus_two() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 1,
        answer: (-2i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page(test);
}

#[test]
fn sbc_zero_page_can_subtract_two_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: (-128i32) as u8,
        operand: 1,
        answer: 127,
        expect_c: true,
        expect_n: false,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_zero_page(test);
}

#[test]
fn sbc_zero_page_can_subtract_a_postitive_and_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: 127,
        operand: (-1i32) as u8,
        answer: 128,
        expect_c: false,
        expect_n: true,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_zero_page(test);
}

#[test]
fn sbc_zero_page_can_subtract_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 3,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page(test);
}

#[test]
fn sbc_zero_page_can_subtract_two_negative_numbers() {
    let test = AdcTestData {
        carry: true,
        a: (-20i32) as u8,
        operand: (-17i32) as u8,
        answer: (-3i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page(test);
}

// SBC Immediate ---------

#[test]
fn sbc_immediate_can_subtract_zero_from_zero_and_get_zero() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 0,
        answer: 0,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_sbc_immediate(test);
}

#[test]
fn sbc_immediate_can_subtract_zero_from_zero_and_carry_and_get_minus_one() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 0,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_immediate(test);
}

#[test]
fn sbc_immediate_can_subtract_one_from_zero_and_get_minus_one() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 1,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_immediate(test);
}

#[test]
fn sbc_immediate_can_subtract_one_from_zero_with_carry_and_get_minus_two() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 1,
        answer: (-2i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_immediate(test);
}

#[test]
fn sbc_immediate_can_subtract_two_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: (-128i32) as u8,
        operand: 1,
        answer: 127,
        expect_c: true,
        expect_n: false,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_immediate(test);
}

#[test]
fn sbc_immediate_can_subtract_a_postitive_and_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: 127,
        operand: (-1i32) as u8,
        answer: 128,
        expect_c: false,
        expect_n: true,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_immediate(test);
}

#[test]
fn sbc_immediate_can_subtract_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 3,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_immediate(test);
}

#[test]
fn sbc_immediate_can_subtract_two_negative_numbers() {
    let test = AdcTestData {
        carry: true,
        a: (-20i32) as u8,
        operand: (-17i32) as u8,
        answer: (-3i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_immediate(test);
}

// SBC Zero Page, X -------

#[test]
fn sbc_zero_page_x_can_subtract_zero_from_zero_and_get_zero() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 0,
        answer: 0,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_sbc_zero_page_x(test);
}

#[test]
fn sbc_zero_page_x_can_subtract_zero_from_zero_and_carry_and_get_minus_one() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 0,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page_x(test);
}

#[test]
fn sbc_zero_page_x_can_subtract_one_from_zero_and_get_minus_one() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 1,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page_x(test);
}

#[test]
fn sbc_zero_page_x_can_subtract_one_from_zero_with_carry_and_get_minus_two() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 1,
        answer: (-2i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page_x(test);
}

#[test]
fn sbc_zero_page_x_can_subtract_two_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: (-128i32) as u8,
        operand: 1,
        answer: 127,
        expect_c: true,
        expect_n: false,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_zero_page_x(test);
}

#[test]
fn sbc_zero_page_x_can_subtract_a_postitive_and_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: 127,
        operand: (-1i32) as u8,
        answer: 128,
        expect_c: false,
        expect_n: true,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_zero_page_x(test);
}

#[test]
fn sbc_zero_page_x_can_subtract_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 3,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page_x(test);
}

#[test]
fn sbc_zero_page_x_can_subtract_two_negative_numbers() {
    let test = AdcTestData {
        carry: true,
        a: (-20i32) as u8,
        operand: (-17i32) as u8,
        answer: (-3i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_zero_page_x(test);
}

// SBC Absolute, X -------

#[test]
fn sbc_absolute_x_can_subtract_zero_from_zero_and_get_zero() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 0,
        answer: 0,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_sbc_absolute_x(test);
}

#[test]
fn sbc_absolute_x_can_subtract_zero_from_zero_and_carry_and_get_minus_one() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 0,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_x(test);
}

#[test]
fn sbc_absolute_x_can_subtract_one_from_zero_and_get_minus_one() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 1,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_x(test);
}

#[test]
fn sbc_absolute_x_can_subtract_one_from_zero_with_carry_and_get_minus_two() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 1,
        answer: (-2i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_x(test);
}

#[test]
fn sbc_absolute_x_can_subtract_two_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: (-128i32) as u8,
        operand: 1,
        answer: 127,
        expect_c: true,
        expect_n: false,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_absolute_x(test);
}

#[test]
fn sbc_absolute_x_can_subtract_a_postitive_and_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: 127,
        operand: (-1i32) as u8,
        answer: 128,
        expect_c: false,
        expect_n: true,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_absolute_x(test);
}

#[test]
fn sbc_absolute_x_can_subtract_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 3,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_x(test);
}

#[test]
fn sbc_absolute_x_can_subtract_two_negative_numbers() {
    let test = AdcTestData {
        carry: true,
        a: (-20i32) as u8,
        operand: (-17i32) as u8,
        answer: (-3i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_x(test);
}

// SBC Absolute, Y

#[test]
fn sbc_absolute_y_can_subtract_zero_from_zero_and_get_zero() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 0,
        answer: 0,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_sbc_absolute_y(test);
}

#[test]
fn sbc_absolute_y_can_subtract_zero_from_zero_and_carry_and_get_minus_one() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 0,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_y(test);
}

#[test]
fn sbc_absolute_y_can_subtract_one_from_zero_and_get_minus_one() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 1,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_y(test);
}

#[test]
fn sbc_absolute_y_can_subtract_one_from_zero_with_carry_and_get_minus_two() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 1,
        answer: (-2i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_y(test);
}

#[test]
fn sbc_absolute_y_can_subtract_two_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: (-128i32) as u8,
        operand: 1,
        answer: 127,
        expect_c: true,
        expect_n: false,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_absolute_y(test);
}

#[test]
fn sbc_absolute_y_can_subtract_a_postitive_and_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: 127,
        operand: (-1i32) as u8,
        answer: 128,
        expect_c: false,
        expect_n: true,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_absolute_y(test);
}

#[test]
fn sbc_absolute_y_can_subtract_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 3,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_y(test);
}

#[test]
fn sbc_absolute_y_can_subtract_two_negative_numbers() {
    let test = AdcTestData {
        carry: true,
        a: (-20i32) as u8,
        operand: (-17i32) as u8,
        answer: (-3i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_absolute_y(test);
}

// SBC Indirect, X ---------

#[test]
fn sbc_indirect_x_can_subtract_zero_from_zero_and_get_zero() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 0,
        answer: 0,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_sbc_indirect_x(test);
}

#[test]
fn sbc_indirect_x_can_subtract_zero_from_zero_and_carry_and_get_minus_one() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 0,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_x(test);
}

#[test]
fn sbc_indirect_x_can_subtract_one_from_zero_and_get_minus_one() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 1,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_x(test);
}

#[test]
fn sbc_indirect_x_can_subtract_one_from_zero_with_carry_and_get_minus_two() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 1,
        answer: (-2i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_x(test);
}

#[test]
fn sbc_indirect_x_can_subtract_two_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: (-128i32) as u8,
        operand: 1,
        answer: 127,
        expect_c: true,
        expect_n: false,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_indirect_x(test);
}

#[test]
fn sbc_indirect_x_can_subtract_a_postitive_and_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: 127,
        operand: (-1i32) as u8,
        answer: 128,
        expect_c: false,
        expect_n: true,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_indirect_x(test);
}

#[test]
fn sbc_indirect_x_can_subtract_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 3,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_x(test);
}

#[test]
fn sbc_indirect_x_can_subtract_two_negative_numbers() {
    let test = AdcTestData {
        carry: true,
        a: (-20i32) as u8,
        operand: (-17i32) as u8,
        answer: (-3i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_x(test);
}

// SBC Indirect, Y -----------

#[test]
fn sbc_indirect_y_can_subtract_zero_from_zero_and_get_zero() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 0,
        answer: 0,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: true,
    };
    test_sbc_indirect_y(test);
}

#[test]
fn sbc_indirect_y_can_subtract_zero_from_zero_and_carry_and_get_minus_one() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 0,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_y(test);
}

#[test]
fn sbc_indirect_y_can_subtract_one_from_zero_and_get_minus_one() {
    let test = AdcTestData {
        carry: true,
        a: 0,
        operand: 1,
        answer: (-1i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_y(test);
}

#[test]
fn sbc_indirect_y_can_subtract_one_from_zero_with_carry_and_get_minus_two() {
    let test = AdcTestData {
        carry: false,
        a: 0,
        operand: 1,
        answer: (-2i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_y(test);
}

#[test]
fn sbc_indirect_y_can_subtract_two_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: (-128i32) as u8,
        operand: 1,
        answer: 127,
        expect_c: true,
        expect_n: false,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_indirect_y(test);
}

#[test]
fn sbc_indirect_y_can_subtract_a_postitive_and_negative_numbers_and_get_signed_overflow() {
    let test = AdcTestData {
        carry: true,
        a: 127,
        operand: (-1i32) as u8,
        answer: 128,
        expect_c: false,
        expect_n: true,
        expect_v: true,
        expect_z: false,
    };
    test_sbc_indirect_y(test);
}

#[test]
fn sbc_indirect_y_can_subtract_two_unsigned_numbers() {
    let test = AdcTestData {
        carry: true,
        a: 20,
        operand: 17,
        answer: 3,
        expect_c: true,
        expect_n: false,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_y(test);
}

#[test]
fn sbc_indirect_y_can_subtract_two_negative_numbers() {
    let test = AdcTestData {
        carry: true,
        a: (-20i32) as u8,
        operand: (-17i32) as u8,
        answer: (-3i32) as u8,
        expect_c: false,
        expect_n: true,
        expect_v: false,
        expect_z: false,
    };
    test_sbc_indirect_y(test);
}
