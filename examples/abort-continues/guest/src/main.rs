//! A guest that calls `std::process::abort()` and still ends successfully, with output `1`.
//!
//! `abort()` compiles to `unimp` (`csrrw x0, cycle, x0`), which RISC-V defines as an
//! illegal instruction: a write to the read-only `cycle` CSR. ZisK transpiles it as a store
//! to the CSR's memory slot and continues with the next instruction. Here that is the
//! next function, a bare `ret`, which returns into `main` after its last call; `ABORTED`
//! is now set, so `main` commits `1` and returns 0.

#![no_main]

ziskos::entrypoint!(main);

static mut ABORTED: bool = false;

fn main() -> i32 {
    if !unsafe { ABORTED } {
        unsafe { ABORTED = true };
        std::process::abort();
    }
    ziskos::io::commit(&1u32);
    0
}
