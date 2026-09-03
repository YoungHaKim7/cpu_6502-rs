use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

#[test]
fn beq_can_branch_forwards_when_zero_is_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    mem[0xFF00] = Cpu::INS_BEQ;
    mem[0xFF01] = 0x1;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF03);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn beq_does_not_branch_forwards_when_zero_is_not_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = false;
    mem[0xFF00] = Cpu::INS_BEQ;
    mem[0xFF01] = 0x1;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF02);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn beq_can_branch_forwards_into_a_new_page_when_zero_is_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFEFD, &mut mem);
    cpu.flags.z = true;
    mem[0xFEFD] = Cpu::INS_BEQ;
    mem[0xFEFE] = 0x1;
    const EXPECTED_CYCLES: i32 = 4;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF00);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn beq_can_branch_backwards_when_zero_is_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFFCC, &mut mem);
    cpu.flags.z = true;
    mem[0xFFCC] = Cpu::INS_BEQ;
    mem[0xFFCD] = (-0x2i32) as u8;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFFCC);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn beq_can_branch_backwards_when_zero_is_set_from_assemble_code() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFFCC, &mut mem);
    cpu.flags.z = true;
    /*
    loop
    lda #0
    beq loop
    */
    mem[0xFFCC] = 0xA9;
    mem[0xFFCC + 1] = 0x00;
    mem[0xFFCC + 2] = 0xF0;
    mem[0xFFCC + 3] = 0xFC;
    const EXPECTED_CYCLES: i32 = 2 + 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFFCC);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn bne_can_branch_forwards_when_zero_is_not_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = false;
    mem[0xFF00] = Cpu::INS_BNE;
    mem[0xFF01] = 0x1;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF03);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn bcs_can_branch_forwards_when_carry_flag_is_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = true;
    mem[0xFF00] = Cpu::INS_BCS;
    mem[0xFF01] = 0x1;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF03);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn bcc_can_branch_forwards_when_carry_flag_is_not_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.c = false;
    mem[0xFF00] = Cpu::INS_BCC;
    mem[0xFF01] = 0x1;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF03);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn bmi_can_branch_forwards_when_negative_flag_is_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_BMI;
    mem[0xFF01] = 0x1;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF03);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn bpl_can_branch_forwards_when_carry_negative_is_not_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_BPL;
    mem[0xFF01] = 0x1;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF03);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn bvs_can_branch_forwards_when_overflow_flag_is_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.v = true;
    mem[0xFF00] = Cpu::INS_BVS;
    mem[0xFF01] = 0x1;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF03);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn bvc_can_branch_forwards_when_overflow_negative_is_not_set() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.v = false;
    mem[0xFF00] = Cpu::INS_BVC;
    mem[0xFF01] = 0x1;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0xFF03);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}
