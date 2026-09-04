# cpu_6502-rs
Learning how a CPU works by emulating one - https://github.com/davepoo/6502Emulator

# A small demo program for the m6502 CPU, in the spirit of the original
- [youtube video](https://youtu.be/qJgsuQoy9bc).

- The `.prg` style program (first two bytes = load address, like a C64 file):

```text
* = $0400
LDA #$42      load the A register with 0x42
JSR $0500     call a subroutine at $0500
STA $10       back from the subroutine: store A at zero page $10

* = $0500
INX           the subroutine increments X...
INX           ...twice
RTS           and returns
```

- Run it with: `cargo run --example demo`


# run

```bash
$ cargo r --example demo

Running...
Done. Cycles used: 21
A: 66 X: 2 Y: 0
PC: 1031 SP: 255
PS: 0
mem[0x0010] = 0x42 (should be 0x42)
X = 2 (the subroutine incremented it twice)
```

# They establish this structure:

```txt
                    crate
                      │
             ┌────────┴────────┐
             │                 │
           cpu                mem
             │                 │
       ┌─────┴─────┐           │
       │           │           │
      Cpu     StatusFlags      Mem
             │
             │
             └──── public re-export ────┐
                                         │
                                  crate::Cpu
                                  crate::StatusFlags
                                  crate::Mem
```
