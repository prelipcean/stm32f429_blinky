//! # stm32f429_blinky
//!
//! A minimal bare-metal Rust application for the STM32F429I-DISC1 board.
//! This file contains the entry point and panic handler.

#![no_std] // Do not link the Rust standard library (required for embedded)
#![no_main] // Disable all Rust-level entry points
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

mod startup_stm32f429zi; // MCU-specific startup code

/// A global mutable counter incremented in the main loop.
static mut COUNTER: u32 = 42;

/// The main entry point for the application.
///
/// # Safety
/// This function is marked unsafe because it accesses a mutable static variable.
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

/// Panic handler for the application.
///
/// Traps the MCU in an infinite loop on panic.
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
