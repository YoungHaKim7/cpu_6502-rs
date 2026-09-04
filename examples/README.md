```rs
6502 hardware concept        Rust implementation
────────────────────────────────────────────────────────
Processor status register    StatusFlags
A register                   Cpu::a
X register                   Cpu::x
Y register                   Cpu::y
Program counter              Cpu::pc
Stack pointer                Cpu::sp
Memory                       Mem
Fetch instruction            fetch_byte()
Fetch 16-bit operand         fetch_word()
Memory read                  read_byte()
Memory write                 write_byte()
Stack address                sp_to_address()
Addressing modes             addr_*
Instruction execution        execute()
```
