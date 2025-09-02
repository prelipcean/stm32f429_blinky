// -----------------------------------------------------------------------------
// STM32F429 PWR (Power Control) utilities
// -----------------------------------------------------------------------------
//
// This module provides constants and helper functions for configuring and
// controlling the Power Control (PWR) peripheral on the STM32F429 microcontroller.
//
// Reference: STM32F429 Reference Manual, section 5 (PWR)
// -----------------------------------------------------------------------------

use crate::bsw::reg_mcu_stm32f429zi::*; // MCU register base addresses and constants
use crate::bsw::reg_utils::*;           // Register access helper functions

// -----------------------------------------------------------------------------
// PWR Register Offsets (relative to PWR base address)
// -----------------------------------------------------------------------------
// These offsets are added to the base address of the PWR peripheral to access
// specific registers.
pub const PWR_CR: u32 = 0x00;  // Power control register
pub const PWR_CSR: u32 = 0x04; // Power control/status register

// -----------------------------------------------------------------------------
// Voltage Regulator Scaling
// -----------------------------------------------------------------------------
/// Sets the regulator voltage scaling for the power control block.
///
/// This controls the internal voltage regulator output voltage, which affects
/// the maximum achievable system frequency and power consumption.
///
/// # Arguments
/// * `scale` - Voltage scale (0..3), written to VOS[1:0] (bits 15:14) of PWR_CR.
///             See reference manual for allowed values.
pub fn pwr_set_regulator_voltage_scale(scale: u32) {
    // The voltage scale is set in bits 15:14 of the PWR_CR register.
    let pwr_cr_addr = (PWR_BASE + PWR_CR) as *mut u32;
    reg_set_bits(pwr_cr_addr, scale & 0x03, 14, 2);
}

// -----------------------------------------------------------------------------
// Overdrive Mode Enable
// -----------------------------------------------------------------------------
/// Enables the overdrive mode required for 180 MHz operation.
///
/// This function enables the overdrive mode and waits for it to become ready.
/// It then enables the overdrive switching and does not return until the
/// hardware is ready.
///
/// Reference: RM0090 Over-drive mode
pub fn pwr_enable_overdrive() {
    // Enable the overdrive mode (ODEN bit, bit 16)
    let pwr_cr_addr = (PWR_BASE + PWR_CR) as *mut u32;
    reg_set_bit(pwr_cr_addr, 16, true);

    // Wait until the overdrive is ready (ODRDY bit, bit 16 in PWR_CSR)
    let pwr_csr_addr = (PWR_BASE + PWR_CSR) as *mut u32;
    let _ = reg_wait_bit(pwr_csr_addr, 16, true, 100_000);

    // Enable the overdrive switching (ODSWEN bit, bit 17)
    let pwr_cr_addr = (PWR_BASE + PWR_CR) as *mut u32;
    reg_set_bit(pwr_cr_addr, 17, true);
}
