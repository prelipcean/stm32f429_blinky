/// Register bit manipulation utilities for STM32F4.
///
/// These are safe wrappers for common register operations.
/// Use with `volatile` pointers for peripheral access.

// AHB1 Bus
pub const RCC_BASE: u32 = 0x4002_3800;
pub const GPIOG_BASE: u32 = 0x4002_1800;

// APB1 Bus
pub const PWR_BASE: u32 = 0x4002_1000;
pub const TIM2_BASE: u32 = 0x4000_0000;

// AHB2 Bus
// (No peripherals used in this part)

// System Bus (ARM Cortex-M4 Core Peripherals)
pub const NVIC_BASE: u32 = 0xE000_E100;

#[inline(always)]
pub fn reg_write(reg: &mut u32, val: u32) {
    unsafe { core::ptr::write_volatile(reg, val) }
}

#[inline(always)]
pub fn reg_read(reg: &u32) -> u32 {
    unsafe { core::ptr::read_volatile(reg) }
}

#[inline(always)]
pub fn reg_set_bit(reg: &mut u32, pos: u8) {
    unsafe { core::ptr::write_volatile(reg, reg_read(reg) | (1 << pos)) }
}

#[inline(always)]
pub fn reg_clr_bit(reg: &mut u32, pos: u8) {
    unsafe { core::ptr::write_volatile(reg, reg_read(reg) & !(1 << pos)) }
}

#[inline(always)]
pub fn reg_read_bit(reg: &u32, pos: u8) -> bool {
    (reg_read(reg) & (1 << pos)) != 0
}

#[inline(always)]
pub fn reg_clr_val(reg: &mut u32, clrmask: u32, pos: u8) {
    unsafe { core::ptr::write_volatile(reg, reg_read(reg) & !(clrmask << pos)) }
}

#[inline(always)]
pub fn reg_set_val(reg: &mut u32, val: u32, setmask: u32, pos: u8) {
    reg_clr_val(reg, setmask, pos);
    unsafe { core::ptr::write_volatile(reg, reg_read(reg) | ((val & setmask) << pos)) }
}

#[inline(always)]
pub fn reg_read_val(reg: &u32, rdmask: u32, pos: u8) -> u32 {
    (reg_read(reg) >> pos) & rdmask
}

/// GPIO pin bitmasks (for 16-bit registers)
pub const GPIO_PIN_0:  u16 = 0x0001;
pub const GPIO_PIN_1:  u16 = 0x0002;
pub const GPIO_PIN_2:  u16 = 0x0004;
pub const GPIO_PIN_3:  u16 = 0x0008;
pub const GPIO_PIN_4:  u16 = 0x0010;
pub const GPIO_PIN_5:  u16 = 0x0020;
pub const GPIO_PIN_6:  u16 = 0x0040;
pub const GPIO_PIN_7:  u16 = 0x0080;
pub const GPIO_PIN_8:  u16 = 0x0100;
pub const GPIO_PIN_9:  u16 = 0x0200;
pub const GPIO_PIN_10: u16 = 0x0400;
pub const GPIO_PIN_11: u16 = 0x0800;
pub const GPIO_PIN_12: u16 = 0x1000;
pub const GPIO_PIN_13: u16 = 0x2000;
pub const GPIO_PIN_14: u16 = 0x4000;
pub const GPIO_PIN_15: u16 = 0x8000;
pub const GPIO_PIN_ALL: u16 = 0xFFFF;