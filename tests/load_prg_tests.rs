use cpu_6502_rs::{Cpu, Mem};

/**
; TestPrg

* = $1000

lda #$FF

start
sta $90
sta $8000
eor #$CC
jmp start

*/
const TEST_PRG: &[u8] = &[
    0x00, 0x10, 0xA9, 0xFF, 0x85, 0x90, 0x8D, 0x00, 0x80, 0x49, 0xCC, 0x4C, 0x02, 0x10,
];

fn setup() -> (Cpu, Mem) {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    (cpu, mem)
}

#[test]
fn test_load_program_a_program_into_the_correct_area_of_memory() {
    // given:
    let (cpu, mut mem) = setup();

    // when:
    cpu.load_prg(TEST_PRG, &mut mem);

    //then:
    assert_eq!(mem[0x0FFF], 0x0);
    assert_eq!(mem[0x1000], 0xA9);
    assert_eq!(mem[0x1001], 0xFF);
    assert_eq!(mem[0x1002], 0x85);
    //....
    assert_eq!(mem[0x1009], 0x4C);
    assert_eq!(mem[0x100A], 0x02);
    assert_eq!(mem[0x100B], 0x10);
    assert_eq!(mem[0x100C], 0x0);
}

#[test]
fn test_load_program_a_program_and_execute_it() {
    // given:
    let (mut cpu, mut mem) = setup();

    // when:
    let start_address = cpu.load_prg(TEST_PRG, &mut mem);
    cpu.pc = start_address;

    //then:
    let mut clock = 1000;
    while clock > 0 {
        clock -= cpu.execute(1, &mut mem);
    }
}

#[test]
fn load_the_6502_test_prg() {
    // (The C++ original has this test compiled out with `#if 0`:
    // loading Klaus2m5's 6502_functional_test.bin and running it forever.)
}
