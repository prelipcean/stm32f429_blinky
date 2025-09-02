// -----------------------------------------------------------------------------
// STM32F429 FLASH (Flash Memory) utilities
// -----------------------------------------------------------------------------

use crate::bsw::reg_mcu_stm32f429zi::*; // MCU register base addresses and constants
use crate::bsw::reg_utils::*; // Register access helper functions

// -----------------------------------------------------------------------------
// FLASH Register Offsets (relative to FLASH base address)
// -----------------------------------------------------------------------------
pub const FLASH_ACR: u32 = 0x00;
pub const FLASH_KEYR: u32 = 0x04;
pub const FLASH_OPTKEYR: u32 = 0x08;
pub const FLASH_SR: u32 = 0x0C;
pub const FLASH_CR: u32 = 0x10;
pub const FLASH_OPTCR: u32 = 0x14;
pub const FLASH_OPTCR1: u32 = 0x18;

/// Sets the number of wait states for the FLASH memory.
///
/// # Arguments
/// * `ws` - Number of wait states (0..15)
pub fn flash_set_wait_states(ws: u32) {
    // The number of wait states is set in the LATENCY[3:0] bits of FLASH_ACR.
    // Only the lower 4 bits are valid.
    let flash_acr_addr = (FLASH_INTERFACE_BASE + FLASH_ACR) as *mut u32;
    reg_set_bits(flash_acr_addr, ws & 0x0F, 0, 4);
}
