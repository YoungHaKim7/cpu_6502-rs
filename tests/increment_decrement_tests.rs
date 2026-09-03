use cpu_6502_rs::{Cpu, Mem};

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

fn expect_unaffected_flags(cpu: &Cpu, cpu_before: &Cpu) {
    assert_eq!(cpu.flags.c, cpu_before.flags.c);
    assert_eq!(cpu.flags.i, cpu_before.flags.i);
    assert_eq!(cpu.flags.d, cpu_before.flags.d);
    assert_eq!(cpu.flags.b, cpu_before.flags.b);
    assert_eq!(cpu.flags.v, cpu_before.flags.v);
}

#[test]
fn inx_can_increment_a_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0x0;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_INX;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0x1);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn inx_can_increment_255() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0xFF;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_INX;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0x0); //NOTE: does this instruction actually wrap?
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn inx_can_increment_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0b1000_1000;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_INX;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0b1000_1001);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn iny_can_increment_a_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.y = 0x0;
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_INY;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.y, 0x1);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn iny_can_increment_255() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.y = 0xFF;
    cpu.flags.z = false;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_INY;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.y, 0x0); //NOTE: does this instruction actually wrap?
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn iny_can_increment_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.y = 0b1000_1000;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_INY;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.y, 0b1000_1001);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dey_can_decement_a_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.y = 0x0;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_DEY;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.y, 0xFF); //NOTE: Does this wrap?
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dey_can_decrement_255() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.y = 0xFF;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_DEY;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.y, 0xFE);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dey_can_decrement_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.y = 0b1000_1001;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_DEY;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.y, 0b1000_1000);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dex_can_decement_a_zero_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0x0;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_DEX;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0xFF); //NOTE: Does this wrap?
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dex_can_decrement_255() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0xFF;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_DEX;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0xFE);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dex_can_decrement_a_negative_value() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.x = 0b1000_1001;
    cpu.flags.z = true;
    cpu.flags.n = false;
    mem[0xFF00] = Cpu::INS_DEX;
    const EXPECTED_CYCLES: i32 = 2;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.x, 0b1000_1000);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dec_can_decrement_a_value_in_the_zero_page() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_DEC_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 0x57;
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0x56);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dec_can_decrement_a_value_in_the_zero_page_x() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_DEC_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 0x57;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0x56);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dec_can_decrement_a_value_absolute() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_DEC_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0x57;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0x56);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn dec_can_decrement_a_value_absolute_x() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_DEC_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 0x57;
    const EXPECTED_CYCLES: i32 = 7;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0x56);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn inc_can_increment_a_value_in_the_zero_page() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_INC_ZP;
    mem[0xFF01] = 0x42;
    mem[0x0042] = 0x57;
    const EXPECTED_CYCLES: i32 = 5;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042], 0x58);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn inc_can_increment_a_value_in_the_zero_page_x() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_INC_ZPX;
    mem[0xFF01] = 0x42;
    mem[0x0042 + 0x10] = 0x57;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0042 + 0x10], 0x58);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn inc_can_increment_a_value_absolute() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    mem[0xFF00] = Cpu::INS_INC_ABS;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = 0x57;
    const EXPECTED_CYCLES: i32 = 6;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000], 0x58);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn inc_can_increment_a_value_absolute_x() {
    // given:
    let (mut cpu, mut mem) = setup();
    cpu.reset_at(0xFF00, &mut mem);
    cpu.flags.z = true;
    cpu.flags.n = true;
    cpu.x = 0x10;
    mem[0xFF00] = Cpu::INS_INC_ABSX;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000 + 0x10] = 0x57;
    const EXPECTED_CYCLES: i32 = 7;
    let cpu_copy = cpu;

    // when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    // then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x8000 + 0x10], 0x58);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    expect_unaffected_flags(&cpu, &cpu_copy);
}

#[test]
fn test_load_a_program_that_can_inc_memory() {
    // given:
    let (mut cpu, mut mem) = setup();

    // when:
    /*
     * = $1000

    lda #00
    sta $42

    start
    inc $42
    ldx $42
    inx
    jmp start
    */
    let test_prg: [u8; 14] = [
        0x0, 0x10, 0xA9, 0x00, 0x85, 0x42, 0xE6, 0x42, 0xA6, 0x42, 0xE8, 0x4C, 0x04, 0x10,
    ];

    let start_address = cpu.load_prg(&test_prg, &mut mem);
    cpu.pc = start_address;

    //then:
    let mut clock: i32 = 1000;
    while clock > 0 {
        clock -= cpu.execute(1, &mut mem);
    }
}
