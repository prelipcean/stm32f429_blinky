#![no_std] // Do not link the Rust standard library (required for embedded)
#![no_main] // Disable all Rust-level entry points
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

mod startup_stm32f429zi; // MCU-specific startup code

// A mutable static variable
static mut COUNTER: u32 = 42;

#[unsafe(no_mangle)]
fn main() -> ! {
    // This is where our application logic will go.

    loop {
        // Increment the counter in an infinite loop.
        // Unsafe is required for mutable statics.
        unsafe {
            COUNTER = COUNTER.wrapping_add(1);
        }
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    // Trap the MCU in an infinite loop on panic.
    loop {}
}
