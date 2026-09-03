use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

#[test]
fn nop_will_do_nothing_but_consume_a_cycle() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_NOP;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.ps(), cpu_copy.ps());
    assert_eq!(cpu.pc, 0xFF01);
    assert_eq!(cpu.a, cpu_copy.a);
    assert_eq!(cpu.x, cpu_copy.x);
    assert_eq!(cpu.y, cpu_copy.y);
    assert_eq!(cpu.sp, cpu_copy.sp);
}

#[test]
fn brk_will_load_the_program_counter_from_the_interrupt_vector() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_BRK;
    mem[0xFFFE] = 0x00;
    mem[0xFFFF] = 0x80;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0x8000);
}

#[test]
fn brk_will_load_the_program_counter_from_the_interrupt_vector2() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_BRK;
    mem[0xFFFE] = 0x00;
    mem[0xFFFF] = 0x90;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.pc, 0x9000);
}

#[test]
fn brk_will_set_the_break_flag() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.b = false;
    mem[0xFF00] = Cpu::INS_BRK;
    const EXPECTED_CYCLES: i32 = 7;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert!(cpu.flags.b);
}

#[test]
fn brk_will_push_3_bytes_onto_the_stack() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_BRK;
    const EXPECTED_CYCLES: i32 = 7;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.sp, cpu_copy.sp.wrapping_sub(3));
}

#[test]
fn brk_will_push_pc_and_ps_onto_the_stack() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_BRK;
    const EXPECTED_CYCLES: i32 = 7;
    let cpu_copy = cpu;
    let old_sp = cpu_copy.sp;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x100 | old_sp as u16], 0xFF);
    // https://www.c64-wiki.com/wiki/BRK
    // Note that since BRK increments the program counter by
    // 2 instead of 1, it is advisable to use a NOP after it
    // to avoid issues
    assert_eq!(mem[(0x100 | old_sp as u16) - 1], 0x02);
    assert_eq!(
        mem[(0x100 | old_sp as u16) - 2],
        cpu_copy.ps() | Cpu::UNUSED_FLAG_BIT | Cpu::BREAK_FLAG_BIT
    );

    // https://wiki.nesdev.com/w/index.php/Status_flags
    // Instruction	|Bits 5 and 4	| Side effects after pushing
    // BRK			|	11			| I is set to 1
    assert!(cpu.flags.i);
}

#[test]
fn rti_can_return_from_an_interrupt_leaving_the_cpu_in_the_state_when_it_entered() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    mem[0xFF00] = Cpu::INS_BRK;
    mem[0xFFFE] = 0x00;
    mem[0xFFFF] = 0x80;
    mem[0x8000] = Cpu::INS_RTI;
    const EXPECTED_CYCLES_BRK: i32 = 7;
    const EXPECTED_CYCLES_RTI: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles_brk = cpu.execute(EXPECTED_CYCLES_BRK, &mut mem);
    let actual_cycles_rti = cpu.execute(EXPECTED_CYCLES_RTI, &mut mem);

    // then:
    assert_eq!(actual_cycles_brk, EXPECTED_CYCLES_BRK);
    assert_eq!(actual_cycles_rti, EXPECTED_CYCLES_RTI);
    assert_eq!(cpu_copy.sp, cpu.sp);
    assert_eq!(cpu.pc, 0xFF02);
    assert_eq!(cpu_copy.ps(), cpu.ps());
}
