use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

fn verify_unmodified_flags_from_logical_op_instruction(cpu: &Cpu, cpu_copy: &Cpu) {
    assert_eq!(cpu.flags.c, cpu_copy.flags.c);
    assert_eq!(cpu.flags.i, cpu_copy.flags.i);
    assert_eq!(cpu.flags.d, cpu_copy.flags.d);
    assert_eq!(cpu.flags.b, cpu_copy.flags.b);
    assert_eq!(cpu.flags.v, cpu_copy.flags.v);
}

#[derive(Clone, Copy)]
enum ELogicalOp {
    And,
    Eor,
    Or,
}

fn do_logical_op(a: u8, b: u8, logical_op: ELogicalOp) -> u8 {
    match logical_op {
        ELogicalOp::And => a & b,
        ELogicalOp::Or => a | b,
        ELogicalOp::Eor => a ^ b,
    }
    // no `throw - 1` needed: the match above is exhaustive
}

fn test_logical_op_immediate(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0xCC;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_IM,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_IM,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_IM,
    }
    mem[0xFFFD] = 0x84;

    //when:
    let cpu_copy = cpu;
    let cycles_used = cpu.execute(2, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x84, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, 2);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_logical_op_zero_page(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0xCC;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_ZP,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_ZP,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_ZP,
    }
    mem[0xFFFD] = 0x42;
    mem[0x0042] = 0x37;

    //when:
    let cpu_copy = cpu;
    let cycles_used = cpu.execute(3, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, 3);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_logical_op_zero_page_x(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0xCC;
    cpu.x = 5;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_ZPX,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_ZPX,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_ZPX,
    }
    mem[0xFFFD] = 0x42;
    mem[0x0047] = 0x37;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(4, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, 4);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_logical_op_absolute(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.a = 0xCC;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_ABS,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_ABS,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_ABS,
    }
    mem[0xFFFD] = 0x80;
    mem[0xFFFE] = 0x44; //0x4480
    mem[0x4480] = 0x37;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_logical_op_absolute_x(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.a = 0xCC;
    cpu.x = 1;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_ABSX,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_ABSX,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_ABSX,
    }
    mem[0xFFFD] = 0x80;
    mem[0xFFFE] = 0x44; //0x4480
    mem[0x4481] = 0x37;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_logical_op_absolute_y(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.a = 0xCC;
    cpu.y = 1;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_ABSY,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_ABSY,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_ABSY,
    }
    mem[0xFFFD] = 0x80;
    mem[0xFFFE] = 0x44; //0x4480
    mem[0x4481] = 0x37;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_load_register_absolute_y_when_crossing_page(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0xCC;
    cpu.y = 0xFF;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_ABSY,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_ABSY,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_ABSY,
    }
    mem[0xFFFD] = 0x02;
    mem[0xFFFE] = 0x44; //0x4402
    mem[0x4501] = 0x37; //0x4402+0xFF crosses page boundary!
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_load_register_absolute_x_when_crossing_page(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0xCC;
    cpu.x = 0xFF;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_ABSX,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_ABSX,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_ABSX,
    }
    mem[0xFFFD] = 0x02;
    mem[0xFFFE] = 0x44; //0x4402
    mem[0x4501] = 0x37; //0x4402+0xFF crosses page boundary!
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_logical_op_indirect_x(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.a = 0xCC;
    cpu.x = 0x04;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_INDX,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_INDX,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_INDX,
    }
    mem[0xFFFD] = 0x02;
    mem[0x0006] = 0x00; //0x2 + 0x4
    mem[0x0007] = 0x80;
    mem[0x8000] = 0x37;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_logical_op_indirect_y(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.a = 0xCC;
    cpu.y = 0x04;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_INDY,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_INDY,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_INDY,
    }
    mem[0xFFFD] = 0x02;
    mem[0x0002] = 0x00;
    mem[0x0003] = 0x80;
    mem[0x8004] = 0x37; //0x8000 + 0x4
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_logical_op_indirect_y_when_it_crosses_a_page(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0xCC;
    cpu.y = 0xFF;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_INDY,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_INDY,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_INDY,
    }
    mem[0xFFFD] = 0x02;
    mem[0x0002] = 0x02;
    mem[0x0003] = 0x80;
    mem[0x8101] = 0x37; //0x8002 + 0xFF
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

fn test_logical_op_zero_page_x_when_it_wraps(logical_op: ELogicalOp) {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0xCC;
    cpu.x = 0xFF;
    match logical_op {
        ELogicalOp::And => mem[0xFFFC] = Cpu::INS_AND_ZPX,
        ELogicalOp::Or => mem[0xFFFC] = Cpu::INS_ORA_ZPX,
        ELogicalOp::Eor => mem[0xFFFC] = Cpu::INS_EOR_ZPX,
    }
    mem[0xFFFD] = 0x80;
    mem[0x007F] = 0x37;

    //when:
    let cpu_copy = cpu;
    let cycles_used = cpu.execute(4, &mut mem);

    //then:
    let expected_result = do_logical_op(0xCC, 0x37, logical_op);
    let expected_negative = (expected_result & 0b1000_0000) > 0;
    assert_eq!(cpu.a, expected_result);
    assert_eq!(cycles_used, 4);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.flags.n, expected_negative);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

#[test]
fn test_logical_op_and_on_a_register_immediate() {
    test_logical_op_immediate(ELogicalOp::And);
}

#[test]
fn test_logical_op_or_on_a_register_immediate() {
    test_logical_op_immediate(ELogicalOp::Or);
}

#[test]
fn test_logical_op_eor_on_a_register_immediate() {
    test_logical_op_immediate(ELogicalOp::Eor);
}

#[test]
fn test_logical_op_and_on_a_register_zero_page() {
    test_logical_op_zero_page(ELogicalOp::And);
}

#[test]
fn test_logical_op_or_on_a_register_zero_page() {
    test_logical_op_zero_page(ELogicalOp::Or);
}

#[test]
fn test_logical_op_eor_on_a_register_zero_page() {
    test_logical_op_zero_page(ELogicalOp::Eor);
}

#[test]
fn test_logical_op_eor_immediate_can_affect_zero_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.a = 0xCC;
    mem[0xFFFC] = Cpu::INS_EOR_IM;
    mem[0xFFFD] = cpu.a;
    let cpu_copy = cpu;

    //when:
    cpu.execute(2, &mut mem);

    //then:
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    verify_unmodified_flags_from_logical_op_instruction(&cpu, &cpu_copy);
}

#[test]
fn test_logical_op_and_on_a_register_zero_page_x() {
    test_logical_op_zero_page_x(ELogicalOp::And);
}

#[test]
fn test_logical_op_or_on_a_register_zero_page_x() {
    test_logical_op_zero_page_x(ELogicalOp::Or);
}

#[test]
fn test_logical_op_eor_on_a_register_zero_page_x() {
    test_logical_op_zero_page_x(ELogicalOp::Eor);
}

#[test]
fn logical_op_eor_can_load_a_value_into_the_a_register_when_it_wraps_zero_page_x() {
    test_logical_op_zero_page_x_when_it_wraps(ELogicalOp::Eor);
}

#[test]
fn logical_op_or_can_load_a_value_into_the_a_register_when_it_wraps_zero_page_x() {
    test_logical_op_zero_page_x_when_it_wraps(ELogicalOp::Or);
}

#[test]
fn logical_op_and_can_load_a_value_into_the_a_register_when_it_wraps_zero_page_x() {
    test_logical_op_zero_page_x_when_it_wraps(ELogicalOp::And);
}

#[test]
fn test_logical_op_eor_on_a_register_absolute() {
    test_logical_op_absolute(ELogicalOp::Eor);
}

#[test]
fn test_logical_op_or_on_a_register_absolute() {
    test_logical_op_absolute(ELogicalOp::Or);
}

#[test]
fn test_logical_op_and_on_a_register_absolute() {
    test_logical_op_absolute(ELogicalOp::And);
}

#[test]
fn test_logical_op_and_on_a_register_absolute_x() {
    test_logical_op_absolute_x(ELogicalOp::And);
}

#[test]
fn test_logical_op_or_on_a_register_absolute_x() {
    test_logical_op_absolute_x(ELogicalOp::Or);
}

#[test]
fn test_logical_op_eor_on_a_register_absolute_x() {
    test_logical_op_absolute_x(ELogicalOp::Eor);
}

#[test]
fn test_logical_op_and_when_it_crosses_a_page_boundary_absolute_x() {
    test_load_register_absolute_x_when_crossing_page(ELogicalOp::And);
}

#[test]
fn test_logical_op_or_when_it_crosses_a_page_boundary_absolute_x() {
    test_load_register_absolute_x_when_crossing_page(ELogicalOp::Or);
}

#[test]
fn test_logical_op_eor_when_it_crosses_a_page_boundary_absolute_x() {
    test_load_register_absolute_x_when_crossing_page(ELogicalOp::Eor);
}

#[test]
fn test_logical_op_and_absolute_y() {
    test_logical_op_absolute_y(ELogicalOp::And);
}

#[test]
fn test_logical_op_or_absolute_y() {
    test_logical_op_absolute_y(ELogicalOp::Or);
}

#[test]
fn test_logical_op_eor_absolute_y() {
    test_logical_op_absolute_y(ELogicalOp::Eor);
}

#[test]
fn test_logical_op_and_when_it_crosses_a_page_boundary_absolute_y() {
    test_load_register_absolute_y_when_crossing_page(ELogicalOp::And);
}

#[test]
fn test_logical_op_or_when_it_crosses_a_page_boundary_absolute_y() {
    test_load_register_absolute_y_when_crossing_page(ELogicalOp::Or);
}

#[test]
fn test_logical_op_eor_when_it_crosses_a_page_boundary_absolute_y() {
    test_load_register_absolute_y_when_crossing_page(ELogicalOp::Eor);
}

#[test]
fn test_logical_op_and_indirect_x() {
    test_logical_op_indirect_x(ELogicalOp::And);
}

#[test]
fn test_logical_op_eor_indirect_x() {
    test_logical_op_indirect_x(ELogicalOp::Eor);
}

#[test]
fn test_logical_op_or_indirect_x() {
    test_logical_op_indirect_x(ELogicalOp::Or);
}

#[test]
fn test_logical_op_and_indirect_y() {
    test_logical_op_indirect_y(ELogicalOp::And);
}

#[test]
fn test_logical_op_or_indirect_y() {
    test_logical_op_indirect_y(ELogicalOp::Or);
}

#[test]
fn test_logical_op_eor_indirect_y() {
    test_logical_op_indirect_y(ELogicalOp::Eor);
}

#[test]
fn test_logical_op_and_when_it_crosses_a_page_indirect_y() {
    test_logical_op_indirect_y_when_it_crosses_a_page(ELogicalOp::And);
}

#[test]
fn test_logical_op_or_when_it_crosses_a_page_indirect_y() {
    test_logical_op_indirect_y_when_it_crosses_a_page(ELogicalOp::Or);
}

#[test]
fn test_logical_op_eor_when_it_crosses_a_page_indirect_y() {
    test_logical_op_indirect_y_when_it_crosses_a_page(ELogicalOp::Eor);
}

#[test]
fn test_bit_zero_page() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.v = false;
    cpu.flags.n = false;
    cpu.a = 0xCC;
    mem[0xFFFC] = Cpu::INS_BIT_ZP;
    mem[0xFFFD] = 0x42;
    mem[0x0042] = 0xCC;
    let _cpu_copy = cpu;
    const EXPECTED_CYCLES: i32 = 3;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0xCC);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.v);
    assert!(cpu.flags.n);
}

#[test]
fn test_bit_zero_page_result_zero() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.v = true;
    cpu.flags.n = true;
    cpu.a = 0xCC;
    mem[0xFFFC] = Cpu::INS_BIT_ZP;
    mem[0xFFFD] = 0x42;
    mem[0x0042] = 0x33;
    let _cpu_copy = cpu;
    const EXPECTED_CYCLES: i32 = 3;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0xCC);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.v);
    assert!(!cpu.flags.n);
}

#[test]
fn test_bit_zero_page_result_zero_bits6_and7_zero() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.v = false;
    cpu.flags.n = false;
    cpu.a = 0x33;
    mem[0xFFFC] = Cpu::INS_BIT_ZP;
    mem[0xFFFD] = 0x42;
    mem[0x0042] = 0xCC;
    let _cpu_copy = cpu;
    const EXPECTED_CYCLES: i32 = 3;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0x33);
    assert!(cpu.flags.z);
    assert!(cpu.flags.v);
    assert!(cpu.flags.n);
}

#[test]
fn test_bit_zero_page_result_zero_bits6_and7_mixed() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.v = false;
    cpu.flags.n = true;
    mem[0xFFFC] = Cpu::INS_BIT_ZP;
    mem[0xFFFD] = 0x42;
    mem[0x0042] = 0b0100_0000;
    const EXPECTED_CYCLES: i32 = 3;

    //when:
    cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert!(cpu.flags.v);
    assert!(!cpu.flags.n);
}

#[test]
fn test_bit_absolute() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.v = false;
    cpu.flags.n = false;
    cpu.a = 0xCC;
    mem[0xFFFC] = Cpu::INS_BIT_ABS;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    mem[0x8000] = 0xCC;
    let _cpu_copy = cpu;
    const EXPECTED_CYCLES: i32 = 4;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0xCC);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.v);
    assert!(cpu.flags.n);
}

#[test]
fn test_bit_absolute_result_zero() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.v = true;
    cpu.flags.n = true;
    cpu.a = 0xCC;
    mem[0xFFFC] = Cpu::INS_BIT_ABS;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    mem[0x8000] = 0x33;
    let _cpu_copy = cpu;
    const EXPECTED_CYCLES: i32 = 4;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0xCC);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.v);
    assert!(!cpu.flags.n);
}

#[test]
fn test_bit_absolute_result_zero_bit6_and7_zero() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.v = false;
    cpu.flags.n = false;
    cpu.a = 0x33;
    mem[0xFFFC] = Cpu::INS_BIT_ABS;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    mem[0x8000] = 0xCC;
    let _cpu_copy = cpu;
    const EXPECTED_CYCLES: i32 = 4;

    //when:
    let cycles_used = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(cycles_used, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0x33);
    assert!(cpu.flags.z);
    assert!(cpu.flags.v);
    assert!(cpu.flags.n);
}

#[test]
fn test_bit_absolute_result_zero_bit6_and7_mixed() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.flags.v = true;
    cpu.flags.n = false;
    mem[0xFFFC] = Cpu::INS_BIT_ABS;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    mem[0x8000] = 0b1000_0000;
    let _cpu_copy = cpu;
    const EXPECTED_CYCLES: i32 = 4;

    //when:
    cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert!(!cpu.flags.v);
    assert!(cpu.flags.n);
}
