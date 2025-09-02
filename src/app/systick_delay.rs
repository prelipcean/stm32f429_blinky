use crate::bsw::reg_cpu_cortex_m4::*;
use crate::bsw::reg_utils::*;

const SYSTICK_CLKSOURCE_POS: u32 = 2;
const SYSTICK_ENABLE_POS: u32 = 0;
const SYSTICK_COUNTFLAG_POS: u32 = 16;

/// Initialize the SysTick timer for 1ms tick (AHB/8 clock source, disabled by default)
pub fn systick_init() {
    unsafe {
        // Disable SysTick
        reg_write(STCSR_BASE as *mut u32, 0x00);
        // Set reload value to max (24 bits)
        reg_write(STRVR_BASE as *mut u32, 0x00FF_FFFF);
        // Clear current value
        reg_write(STCVR_BASE as *mut u32, 0x00);

        // Select AHB/8 as clock source (clear CLKSOURCE bit, bit 2)
        reg_clear_bit(STCSR_BASE as *mut u32, SYSTICK_CLKSOURCE_POS);
        // Enable SysTick (set ENABLE bit, bit 0)
        reg_set_bit(STCSR_BASE as *mut u32, SYSTICK_ENABLE_POS, true);
    }
}

/// Delay for approximately 1 millisecond (assuming 180 MHz system clock, AHB/8)
pub fn delay_one_ms() {
    unsafe {
        // Set reload for 1ms: 180_000_000 / 8 / 1000 = 22_500
        reg_write(STRVR_BASE as *mut u32, 22_500 - 1);
        reg_write(STCVR_BASE as *mut u32, 0x00);

        // Wait for COUNTFLAG (bit 16) to be set
        while !reg_read_bit(STCSR_BASE as *mut u32, SYSTICK_COUNTFLAG_POS) {}
    }
}

/// Delay for t milliseconds
pub fn delay_ms(mut t: u32) {
    while t > 0 {
        delay_one_ms();
        t -= 1;
    }
}
