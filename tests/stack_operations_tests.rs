use cpu_6502_rs::{Cpu, Mem, StatusFlags};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

#[test]
fn tsx_can_transfer_the_stack_pointer_to_x_register() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x00;
    cpu.sp = 0x01;
    mem[0xFF00] = Cpu::INS_TSX;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0x01);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn tsx_can_transfer_a_zero_stack_pointer_to_x_register() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x00;
    cpu.sp = 0x00;
    mem[0xFF00] = Cpu::INS_TSX;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0x00);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
}

#[test]
fn tsx_can_transfer_a_negative_stack_pointer_to_x_register() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = false;
    cpu.flags.n = false;
    cpu.x = 0x00;
    cpu.sp = 0b1000_0000;
    mem[0xFF00] = Cpu::INS_TSX;
    const EXPECTED_CYCLES: i32 = 2;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0b1000_0000);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn txs_can_transfer_x_register_to_the_stack_pointer() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0xFF;
    cpu.sp = 0;
    mem[0xFF00] = Cpu::INS_TXS;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.sp, 0xFF);
    assert_eq!(cpu.ps(), cpu_copy.ps());
}

#[test]
fn pha_can_push_a_regsiter_onto_the_stack() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.a = 0x42;
    mem[0xFF00] = Cpu::INS_PHA;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[cpu.sp_to_address() + 1], cpu.a);
    assert_eq!(cpu.ps(), cpu_copy.ps());
    assert_eq!(cpu.sp, 0xFE);
}

#[test]
fn pla_can_pull_a_value_from_the_stack_into_the_a_regsiter() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.a = 0x00;
    cpu.sp = 0xFE;
    mem[0x01FF] = 0x42;
    mem[0xFF00] = Cpu::INS_PLA;
    const EXPECTED_CYCLES: i32 = 4;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.sp, 0xFF);
}

#[test]
fn pla_can_pull_a_zero_value_from_the_stack_into_the_a_regsiter() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = false;
    cpu.flags.n = true;
    cpu.a = 0x42;
    cpu.sp = 0xFE;
    mem[0x01FF] = 0x00;
    mem[0xFF00] = Cpu::INS_PLA;
    const EXPECTED_CYCLES: i32 = 4;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0x00);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert_eq!(cpu.sp, 0xFF);
}

#[test]
fn pla_can_pull_a_negative_value_from_the_stack_into_the_a_regsiter() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.n = false;
    cpu.flags.z = true;
    cpu.a = 0x42;
    cpu.sp = 0xFE;
    mem[0x01FF] = 0b1000_0001;
    mem[0xFF00] = Cpu::INS_PLA;
    const EXPECTED_CYCLES: i32 = 4;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a, 0b1000_0001);
    assert!(cpu.flags.n);
    assert!(!cpu.flags.z);
    assert_eq!(cpu.sp, 0xFF);
}

#[test]
fn php_can_push_processor_status_onto_the_stack() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags = StatusFlags::from_byte(0xCC);
    mem[0xFF00] = Cpu::INS_PHP;
    const EXPECTED_CYCLES: i32 = 3;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(
        mem[cpu.sp_to_address() + 1],
        0xCC | Cpu::UNUSED_FLAG_BIT | Cpu::BREAK_FLAG_BIT
    );
    assert_eq!(cpu.ps(), cpu_copy.ps());
    assert_eq!(cpu.sp, 0xFE);
}

#[test]
fn php_always_sets_bits4_and5_on_the_stack() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags = StatusFlags::from_byte(0x0);
    mem[0xFF00] = Cpu::INS_PHP;
    const EXPECTED_CYCLES: i32 = 3;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    let add_ps_on_stack = cpu.sp_to_address() + 1;
    assert_eq!(actual_cycles, EXPECTED_CYCLES);

    // https://wiki.nesdev.com/w/index.php/Status_flags
    //Two interrupts (/IRQ and /NMI) and two instructions (PHP and BRK) push
    // the flags to the stack. In the byte pushed, bit 5 is always set to 1,
    //and bit 4 is 1 if from an instruction (PHP or BRK) or 0 if from an
    // interrupt line being pulled low (/IRQ or /NMI). This is the only time
    // and place where the B flag actually exists: not in the status register
    // itself, but in bit 4 of the copy that is written to the stack.
    const FLAGS_ON_STACK: u8 = Cpu::UNUSED_FLAG_BIT | Cpu::BREAK_FLAG_BIT;
    assert_eq!(mem[add_ps_on_stack], FLAGS_ON_STACK);
}

#[test]
fn plp_can_pull_a_value_from_the_stack_into_the_processor_status() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.sp = 0xFE;
    cpu.flags = StatusFlags::from_byte(0);
    mem[0x01FF] = 0x42 | Cpu::BREAK_FLAG_BIT | Cpu::UNUSED_FLAG_BIT;
    mem[0xFF00] = Cpu::INS_PLP;
    const EXPECTED_CYCLES: i32 = 4;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.ps(), 0x42);
}

#[test]
fn plp_clears_bits4_and5_when_pulling_from_the_stack() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.sp = 0xFE;
    cpu.flags = StatusFlags::from_byte(0);
    mem[0x01FF] = Cpu::BREAK_FLAG_BIT | Cpu::UNUSED_FLAG_BIT;
    mem[0xFF00] = Cpu::INS_PLP;
    const EXPECTED_CYCLES: i32 = 4;
    let _cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.ps(), 0);
}
