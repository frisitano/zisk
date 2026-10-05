# Failed executions that prove successfully

Two guests whose executions fail, yet ZisK ends them successfully and proves them: the proof
verifies, with the output each committed before failing (`1`). The zkvm-standards
[Termination Semantics](https://github.com/eth-act/zkevm-standards/tree/main/standards/standard-termination-semantics)
standard requires that no proof of a failed execution verifies.

## `abort-continues`: `std::process::abort()` does not stop the program

`abort()` compiles to `unimp`, the encoding of `csrrw x0, cycle, x0`. RISC-V defines it as an
illegal instruction (a write to the read-only `cycle` CSR), and compilers emit it for every trap:
`core::intrinsics::abort`, Rust's `panic = "abort"` (`__rust_abort`), `__builtin_trap` and
`llvm.trap`. ZisK transpiles it as a store to the CSR's memory slot
(`transpilers/riscv/src/riscv2zisk_context.rs`, `csrrw`) and continues with the next
instruction. In this guest that is the next function, a bare `ret`, which returns into `main`;
`main` then commits `1` and returns 0.

`panic!` and `assert!` reach the same `unimp` in `__rust_abort`. Whether the execution then stops
depends on the code that follows it in the binary.

## `nonzero-exit`: the exit code is ignored

`main` commits `1` and returns `1`. `_start` passes the code to the exit call (`ecall` with
`a7 = 93`, `a0 = 1`), but the exit handler (`add_entry_exit_jmp`) checks only `a7`, publishes the
output and ends the execution successfully.

## Reproduce

With `cargo-zisk` 1.3.1-alpha and the 1.3.1-alpha proving key, from a guest's directory:

```bash
cargo-zisk build --release
ziskemu -e target/elf/riscv64ima-zisk-zkvm-elf/release/<guest> -c   # output 00000001, no error
cargo-zisk prove -e target/elf/riscv64ima-zisk-zkvm-elf/release/<guest> -k <proving-key> -o proof -y
```

## Possible fixes

- Trap on writes to the read-only counter CSRs (`0xC00`-`0xC1F`, which include `unimp`): transpile
  them to `halt`, as reserved encodings already are. ZisK's own precompile CSRs are unaffected.
- End the exit call with a non-zero `a0` in `halt`, or make the exit code a public value the
  verifier checks.
