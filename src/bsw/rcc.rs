// -----------------------------------------------------------------------------
// STM32F429 RCC (Reset and Clock Control) utilities
// -----------------------------------------------------------------------------

use crate::bsw::reg_mcu_stm32f429zi::*; // MCU register base addresses and constants
use crate::bsw::reg_utils::*; // Register access helper functions

// See: reference manual ch7.3 RCC registers page 226

// RCC register offsets (relative to RCC_BASE)
pub const RCC_CR: u32 = 0x00; // Clock control register
pub const RCC_PLLCFGR: u32 = 0x04; // PLL configuration register
pub const RCC_CFGR: u32 = 0x08; // Clock configuration register
pub const RCC_CIR: u32 = 0x0C; // Clock interrupt register
pub const RCC_AHB1RSTR: u32 = 0x10; // AHB1 peripheral reset register
pub const RCC_AHB2RSTR: u32 = 0x14; // AHB2 peripheral reset register
pub const RCC_AHB3RSTR: u32 = 0x18; // AHB3 peripheral reset register
pub const RCC_APB1RSTR: u32 = 0x20; // APB1 peripheral reset register
pub const RCC_APB2RSTR: u32 = 0x24; // APB2 peripheral reset register
pub const RCC_AHB1ENR: u32 = 0x30; // AHB1 peripheral clock enable register
pub const RCC_AHB2ENR: u32 = 0x34; // AHB2 peripheral clock enable register
pub const RCC_AHB3ENR: u32 = 0x38; // AHB3 peripheral clock enable register
pub const RCC_APB1ENR: u32 = 0x40; // APB1 peripheral clock enable register
pub const RCC_APB2ENR: u32 = 0x44; // APB2 peripheral clock enable register
pub const RCC_AHB1LPENR: u32 = 0x50; // AHB1 low power clock enable register
pub const RCC_AHB2LPENR: u32 = 0x54; // AHB2 low power clock enable register
pub const RCC_AHB3LPENR: u32 = 0x58; // AHB3 low power clock enable register
pub const RCC_APB1LPENR: u32 = 0x60; // APB1 low power clock enable register
pub const RCC_APB2LPENR: u32 = 0x64; // APB2 low power clock enable register
pub const RCC_BDCR: u32 = 0x70; // Backup domain control register
pub const RCC_CSR: u32 = 0x74; // Control/status register
pub const RCC_SSCGR: u32 = 0x80; // Spread spectrum clock generation register
pub const RCC_PLLI2SCFGR: u32 = 0x84; // PLLI2S configuration register

/// Enables the clock for a specific GPIO port.
///
/// This must be called before you use any pins on the port.
///
/// # Arguments
/// * `port` - The base address of the GPIO port (e.g., `GPIOA_BASE`).
///
/// # Example
/// ```
/// rcc_enable_gpio_clock(GPIOA_BASE);
/// ```
pub fn rcc_enable_gpio_clock(port: u32) {
    // RCC_AHB1ENR controls the clock gating for all GPIO ports on AHB1
    let rcc_ahb1enr_addr = (RCC_BASE + RCC_AHB1ENR) as *mut u32;

    match port {
        GPIOA_BASE => {
            // Enable the 0th bit of the RCC AHB1ENR register for GPIOA
            reg_set_bit(rcc_ahb1enr_addr, 0, true);
        }
        GPIOB_BASE => {
            // Enable the 1st bit of the RCC AHB1ENR register for GPIOB
            reg_set_bit(rcc_ahb1enr_addr, 1, true);
        }
        GPIOC_BASE => {
            // Enable the 2nd bit of the RCC AHB1ENR register for GPIOC
            reg_set_bit(rcc_ahb1enr_addr, 2, true);
        }
        GPIOD_BASE => {
            // Enable the 3rd bit of the RCC AHB1ENR register for GPIOD
            reg_set_bit(rcc_ahb1enr_addr, 3, true);
        }
        GPIOE_BASE => {
            // Enable the 4th bit of the RCC AHB1ENR register for GPIOE
            reg_set_bit(rcc_ahb1enr_addr, 4, true);
        }
        GPIOF_BASE => {
            // Enable the 5th bit of the RCC AHB1ENR register for GPIOF
            reg_set_bit(rcc_ahb1enr_addr, 5, true);
        }
        GPIOG_BASE => {
            // Enable the 6th bit of the RCC AHB1ENR register for GPIOG
            reg_set_bit(rcc_ahb1enr_addr, 6, true);
        }
        GPIOH_BASE => {
            // Enable the 7th bit of the RCC AHB1ENR register for GPIOH
            reg_set_bit(rcc_ahb1enr_addr, 7, true);
        }
        GPIOI_BASE => {
            // Enable the 8th bit of the RCC AHB1ENR register for GPIOI
            reg_set_bit(rcc_ahb1enr_addr, 8, true);
        }
        GPIOJ_BASE => {
            // Enable the 9th bit of the RCC AHB1ENR register for GPIOJ
            reg_set_bit(rcc_ahb1enr_addr, 9, true);
        }
        GPIOK_BASE => {
            // Enable the 10th bit of the RCC AHB1ENR register for GPIOK
            reg_set_bit(rcc_ahb1enr_addr, 10, true);
        }
        // Add more ports here if needed (GPIOL_BASE, etc.)
        _ => {
            // Unknown port: do nothing
        }
    }
}
