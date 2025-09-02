//! # stm32f429_blinky
//!
//! A minimal bare-metal Rust application for the STM32F429I-DISC1 board.

#![no_std] // Do not link the Rust standard library (required for embedded)
#![no_main] // Disable all Rust-level entry points
#![allow(clippy::empty_loop)]
#![allow(dead_code)]

use crate::app::led::*;
use crate::app::system_clock::*;
use core::panic::PanicInfo;

mod app;
mod bsw;

/// The main entry point for the application.
///
/// # Safety
/// This function is marked unsafe because it accesses a mutable static variable.
#[unsafe(no_mangle)]
fn main() -> ! {
    system_clock_setup();
    system_clock_output_pa8();
    led_init();
    led_on();

    loop {}
}

/// Panic handler for the application.
///
/// Traps the MCU in an infinite loop on panic.
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
