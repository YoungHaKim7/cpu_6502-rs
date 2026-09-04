# 6502_CPU

Great — now the structure is much clearer. I inspected the supplied `cpu.rs` content and the `mem.rs` portion. The CPU implementation contains the important pieces that `lib.rs` should introduce: `StatusFlags`, `Cpu`, the 6502 register set, opcode groups, fetch/read/write operations, stack operations, addressing modes, instruction execution, and cycle accounting.  

One important point: the `cpu.rs` you supplied is **not empty**; the attachment contains its implementation. In particular, the addressing-mode implementation continues through `addr_indirect_y_6`, and `execute()` is the central fetch/decode/execute loop.  

I would therefore make `lib.rs` the **architecture and learning guide for the entire emulator**, while keeping the implementation-specific, line-by-line explanations in `cpu.rs` and `mem.rs`.


### A couple of important documentation choices

I deliberately **didn't put every opcode into `lib.rs`**. Your `cpu.rs` already contains a large opcode table — `LDA`, `LDX`, `LDY`, `STA`, `AND`, `ORA`, `EOR`, branches, arithmetic, comparisons, shifts, stack operations, etc. 

Putting another complete opcode table in `lib.rs` would duplicate the source and make the crate-level documentation harder to navigate. Instead, `lib.rs` should explain the **concept**, while `cpu.rs` should explain the **individual implementation**.

There is also a particularly useful correspondence in your implementation:

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

For example, `fetch_byte()` reads `memory[self.pc]`, increments `PC` with wrapping arithmetic, and consumes one cycle. 

Likewise, the addressing-mode functions aren't just helper functions—they are a major part of the **6502 architecture**. Your implementation explicitly handles zero-page wrapping, indexed addressing, page-boundary detection, indirect addressing, and the special cycle behavior needed by some addressing modes. 

One thing I would **not** silently describe as fully hardware-accurate in the documentation is decimal-mode `ADC`: your source explicitly asserts that decimal mode has not been handled.  Similarly, your `JMP (indirect)` implementation has a TODO concerning the original 6502's page-boundary quirk. 

That distinction is valuable in emulator documentation: **document what the source actually implements, and explicitly identify hardware behaviors that are intentionally simplified or still incomplete.**
