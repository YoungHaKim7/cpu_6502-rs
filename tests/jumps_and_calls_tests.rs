use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

#[test]
fn can_jump_to_a_subroutine_and_jump_back_again() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_JSR;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = Cpu::INS_RTS;
    mem[0xFF03] = Cpu::INS_LDA_IM;
    mem[0xFF04] = 0x42;
    const EXPECTED_CYCLES: i32 = 6 + 6 + 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.sp, cpu_copy.sp);
}

#[test]
fn jsr_does_not_affect_the_processor_status() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_JSR;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.ps(), cpu_copy.ps());
    assert_ne!(cpu.sp, cpu_copy.sp);
    assert_eq!(cpu.pc, 0x8000);
}

#[test]
fn rts_does_not_affect_the_processor_status() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_JSR;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = Cpu::INS_RTS;
    const EXPECTED_CYCLES: i32 = 6 + 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.ps(), cpu_copy.ps());
    assert_eq!(cpu.pc, 0xFF03);
}

#[test]
fn jump_absolute_can_jump_to_an_new_location_in_the_program() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_JMP_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.ps(), cpu_copy.ps());
    assert_eq!(cpu.sp, cpu_copy.sp);
    assert_eq!(cpu.pc, 0x8000);
}

#[test]
fn jump_indirect_can_jump_to_an_new_location_in_the_program() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_JMP_IND;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0x00;
    mem[0x8001] = 0x90;
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.ps(), cpu_copy.ps());
    assert_eq!(cpu.sp, cpu_copy.sp);
    assert_eq!(cpu.pc, 0x9000);
}
