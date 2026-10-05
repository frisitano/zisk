//! A guest whose `main` returns 1, a failed execution, and still ends successfully, with
//! output `1`.
//!
//! `_start` passes `main`'s return value to the exit call (`ecall` with `a7 = 93`,
//! `a0 = 1`), but the exit handler checks only `a7` and ends the execution successfully.

#![no_main]

ziskos::entrypoint!(main);

fn main() -> i32 {
    ziskos::io::commit(&1u32);
    1
}
