// -----------------------------------------------------------------------------
// STM32F429 GPIO (General Purpose Input/Output) utilities
// -----------------------------------------------------------------------------

// GPIO pin numbers
pub const GPIO_PIN_0: u32 = 0;
pub const GPIO_PIN_1: u32 = 1;
pub const GPIO_PIN_2: u32 = 2;
pub const GPIO_PIN_3: u32 = 3;
pub const GPIO_PIN_4: u32 = 4;
pub const GPIO_PIN_5: u32 = 5;
pub const GPIO_PIN_6: u32 = 6;
pub const GPIO_PIN_7: u32 = 7;
pub const GPIO_PIN_8: u32 = 8;
pub const GPIO_PIN_9: u32 = 9;
pub const GPIO_PIN_10: u32 = 10;
pub const GPIO_PIN_11: u32 = 11;
pub const GPIO_PIN_12: u32 = 12;
pub const GPIO_PIN_13: u32 = 13;
pub const GPIO_PIN_14: u32 = 14;
pub const GPIO_PIN_15: u32 = 15;

/// GPIO pin bitmasks for 16‑bit registers (pins 0..15)
pub const GPIO_PIN_0_MASK:  u16 = 1 << 0;
pub const GPIO_PIN_1_MASK:  u16 = 1 << 1;
pub const GPIO_PIN_2_MASK:  u16 = 1 << 2;
pub const GPIO_PIN_3_MASK:  u16 = 1 << 3;
pub const GPIO_PIN_4_MASK:  u16 = 1 << 4;
pub const GPIO_PIN_5_MASK:  u16 = 1 << 5;
pub const GPIO_PIN_6_MASK:  u16 = 1 << 6;
pub const GPIO_PIN_7_MASK:  u16 = 1 << 7;
pub const GPIO_PIN_8_MASK:  u16 = 1 << 8;
pub const GPIO_PIN_9_MASK:  u16 = 1 << 9;
pub const GPIO_PIN_10_MASK: u16 = 1 << 10;
pub const GPIO_PIN_11_MASK: u16 = 1 << 11;
pub const GPIO_PIN_12_MASK: u16 = 1 << 12;
pub const GPIO_PIN_13_MASK: u16 = 1 << 13;
pub const GPIO_PIN_14_MASK: u16 = 1 << 14;
pub const GPIO_PIN_15_MASK: u16 = 1 << 15;