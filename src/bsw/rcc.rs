// -----------------------------------------------------------------------------
// STM32F429 RCC (Reset and Clock Control) utilities
// -----------------------------------------------------------------------------
//
// This module provides constants and helper functions for configuring and
// controlling the Reset and Clock Control (RCC) peripheral on the STM32F429
// microcontroller.
//
// Reference: STM32F429 Reference Manual, section 7.3 (RCC registers, page 226)
// -----------------------------------------------------------------------------

use crate::bsw::reg_mcu_stm32f429zi::*; // MCU register base addresses and constants
use crate::bsw::reg_utils::*; // Register access helper functions

// -----------------------------------------------------------------------------
// RCC Register Offsets (relative to RCC_BASE)
// -----------------------------------------------------------------------------
// These offsets are added to the base address of the RCC peripheral to access
// specific registers.
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

// -----------------------------------------------------------------------------
// Enable GPIO Port Clock
// -----------------------------------------------------------------------------
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

// -----------------------------------------------------------------------------
// Enable Power and SYSCFG Peripheral Clocks
// -----------------------------------------------------------------------------
pub fn rcc_enable_power_clock() {
    // Enable the power interface clock (bit 28 in APB1ENR)
    let rcc_apb1enr_addr = (RCC_BASE + RCC_APB1ENR) as *mut u32;
    reg_set_bit(rcc_apb1enr_addr, 28, true);
}

pub fn rcc_enable_syscfg_clock() {
    // Enable the system configuration controller clock (bit 14 in APB2ENR)
    let rcc_apb2enr_addr = (RCC_BASE + RCC_APB2ENR) as *mut u32;
    reg_set_bit(rcc_apb2enr_addr, 14, true);
}

// -----------------------------------------------------------------------------
// Configure Main PLL for 180 MHz SYSCLK
// -----------------------------------------------------------------------------
/// Configures the main PLL to generate a 180 MHz system clock (SYSCLK).
///
/// This sets up the PLL multipliers/dividers and bus prescalers, enables the PLL,
/// waits for it to lock, and switches SYSCLK to the PLL output.
pub fn rcc_configure_pll_180mhz() {
    // Set PLLM (bits 0..5) to 8 (input divider)
    let pllcfgr_addr = (RCC_BASE + RCC_PLLCFGR) as *mut u32;
    reg_set_bits(pllcfgr_addr, 8, 0, 6);

    // Set PLLN (bits 6..14) to 180 (VCO multiplier)
    reg_set_bits(pllcfgr_addr, 180, 6, 9);

    // Set PLLP (bits 16..17) to 00 (÷2, main system clock output)
    reg_set_bits(pllcfgr_addr, 0, 16, 2);

    // Configure AHB and APBx prescalers
    let cfgr_addr = (RCC_BASE + RCC_CFGR) as *mut u32;

    // Set AHB prescaler (bits 4..7) to 0 (SYSCLK / 1)
    reg_set_bits(cfgr_addr, 0, 4, 4);

    // Set APB1 prescaler (bits 10..12) to 0b101 (HCLK / 4)
    reg_set_bits(cfgr_addr, 0b101, 10, 3);

    // Set APB2 prescaler (bits 13..15) to 0b100 (HCLK / 2)
    reg_set_bits(cfgr_addr, 0b100, 13, 3);

    // Turn on main PLL (set PLLON, bit 24)
    let cr_addr = (RCC_BASE + RCC_CR) as *mut u32;
    reg_set_bit(cr_addr, 24, true);

    // Wait until PLL is ready (PLLRDY, bit 25)
    let _ = reg_wait_bit(cr_addr, 25, true, 100_000);

    // Switch SYSCLK source to PLL (SW bits 0..1 = 0b10)
    reg_set_bits(cfgr_addr, 0b10, 0, 2);

    // Wait until SYSCLK source is PLL (SWS bits 2..3 = 0b10)
    // Use reg_wait_bits to poll for SWS == 0b10 with a timeout
    let switched = reg_wait_bits(
        cfgr_addr, // Register address
        0b10,      // Expected value
        0x3,       // Mask for 2 bits (0b11)
        2,         // Bit position (SWS starts at bit 2)
        100_000,   // Timeout cycles
    );
    if !switched {
        // Optionally handle timeout error here
        // e.g., panic!("SYSCLK switch to PLL failed");
    }

    // PLLSAI not needed for this board
}

// -----------------------------------------------------------------------------
// MCO (Microcontroller Clock Output) Configuration
// -----------------------------------------------------------------------------
/// MCO1 clock source selection for output on PA8.
///
/// You can output HSI, LSE, HSE, or PLL clock to the MCO1 pin (PA8).
/// See RM0090 Table 14 for details.
pub enum McoSource {
    HSI = 0b00, // High-speed internal clock
    LSE = 0b01, // Low-speed external clock
    HSE = 0b10, // High-speed external clock
    PLL = 0b11, // PLL clock
}

/// MCO prescaler division factors.
pub enum Div {
    Div1 = 0b000, // No division
    Div2 = 0b100, // Divide by 2
    Div3 = 0b101, // Divide by 3
    Div4 = 0b110, // Divide by 4
    Div5 = 0b111, // Divide by 5
}

// -----------------------------------------------------------------------------
// Enable MCO1 Output on PA8
// -----------------------------------------------------------------------------
/// Configures and enables the MCO1 output on PA8.
///
/// # Arguments
/// * `mco_source` - The clock source to output (see `McoSource`)
/// * `prescaler`  - The prescaler division factor (see `Div`)
///
/// This function sets the MCO1 source and prescaler in RCC_CFGR.
/// You must also configure PA8 as alternate function (AF0) in GPIO.
pub fn rcc_enable_mco1_output(mco_source: McoSource, prescaler: Div) {
    let rcc_cfgr = (RCC_BASE + RCC_CFGR) as *mut u32;
    reg_set_bits(rcc_cfgr, mco_source as u32, 21, 2); // Set MCO1 source (bits 22:21)
    reg_set_bits(rcc_cfgr, prescaler as u32, 24, 3); // Set MCO1 prescaler (bits 26:24)
}
