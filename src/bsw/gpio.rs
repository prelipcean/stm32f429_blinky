// -----------------------------------------------------------------------------
// STM32F429 GPIO (General Purpose Input/Output) utilities
// -----------------------------------------------------------------------------
//
// This module provides constants and helper functions for configuring and
// controlling GPIO pins on the STM32F429 microcontroller.
//
// Reference: STM32F429 Reference Manual, section 8.4 (GPIO registers, page 284)
// -----------------------------------------------------------------------------

use crate::bsw::reg_mcu_stm32f429zi::*; // MCU register base addresses and constants
use crate::bsw::reg_utils::*; // Register access helper functions

// -----------------------------------------------------------------------------
// GPIO Pin Numbers (0..15)
// -----------------------------------------------------------------------------
// These constants represent the pin numbers for each GPIO pin in a port.
// Useful for passing as arguments to GPIO functions.
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

// -----------------------------------------------------------------------------
// GPIO Pin Bitmasks for 16‑bit Registers (pins 0..15)
// -----------------------------------------------------------------------------
// These bitmask constants are useful for register operations that use a bit per pin,
// such as ODR (output data), IDR (input data), and BSRR (bit set/reset).
pub const GPIO_PIN_0_MASK: u16 = 1 << 0;
pub const GPIO_PIN_1_MASK: u16 = 1 << 1;
pub const GPIO_PIN_2_MASK: u16 = 1 << 2;
pub const GPIO_PIN_3_MASK: u16 = 1 << 3;
pub const GPIO_PIN_4_MASK: u16 = 1 << 4;
pub const GPIO_PIN_5_MASK: u16 = 1 << 5;
pub const GPIO_PIN_6_MASK: u16 = 1 << 6;
pub const GPIO_PIN_7_MASK: u16 = 1 << 7;
pub const GPIO_PIN_8_MASK: u16 = 1 << 8;
pub const GPIO_PIN_9_MASK: u16 = 1 << 9;
pub const GPIO_PIN_10_MASK: u16 = 1 << 10;
pub const GPIO_PIN_11_MASK: u16 = 1 << 11;
pub const GPIO_PIN_12_MASK: u16 = 1 << 12;
pub const GPIO_PIN_13_MASK: u16 = 1 << 13;
pub const GPIO_PIN_14_MASK: u16 = 1 << 14;
pub const GPIO_PIN_15_MASK: u16 = 1 << 15;

// -----------------------------------------------------------------------------
// GPIO Register Offsets (relative to GPIOx base address)
// -----------------------------------------------------------------------------
// These offsets are added to the base address of a GPIO port to access specific registers.
pub const GPIOX_MODER: u32 = 0x00; // GPIO port mode register
pub const GPIOX_OTYPER: u32 = 0x04; // GPIO port output type register
pub const GPIOX_OSPEEDR: u32 = 0x08; // GPIO port output speed register
pub const GPIOX_PUPDR: u32 = 0x0C; // GPIO port pull-up/pull-down register
pub const GPIOX_IDR: u32 = 0x10; // GPIO port input data register
pub const GPIOX_ODR: u32 = 0x14; // GPIO port output data register
pub const GPIOX_BSRR: u32 = 0x18; // GPIO port bit set/reset register
pub const GPIOX_LCKR: u32 = 0x1C; // GPIO port configuration lock register
pub const GPIOX_AFRL: u32 = 0x20; // GPIO alternate function low register
pub const GPIOX_AFRH: u32 = 0x24; // GPIO alternate function high register

// -----------------------------------------------------------------------------
// GPIO Mode Enumeration
// -----------------------------------------------------------------------------
// This enum represents the possible modes for a GPIO pin.
// The #[repr(u32)] attribute ensures the enum is represented as a 32-bit value,
// matching the hardware register layout for safe casting.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GpioMode {
    /// Input mode (reset state) - 0b00
    Input = 0,
    /// General purpose output mode - 0b01
    Output = 1,
    /// Alternate function mode - 0b10
    Alternate = 2,
    /// Analog mode - 0b11
    Analog = 3,
}

// -----------------------------------------------------------------------------
// GPIO Output Type Enumeration
// -----------------------------------------------------------------------------
// This enum selects the output driver type for a GPIO pin.
pub enum GpioType {
    /// Output push-pull (reset state) - 0
    PushPull = 0,
    /// Output open-drain - 1
    OpenDrain = 1,
}

// -----------------------------------------------------------------------------
// GPIO PinState Type Enumeration
// -----------------------------------------------------------------------------
// This enum is used when we set a pin high, low, or toggle it.
pub enum PinState {
    /// The pin is set to a high state (logic 1, usually 3.3V or 5V).
    High,
    /// The pin is set to a low state (logic 0, usually 0V).
    Low,
    /// The pin state is toggled (from high to low, or from low to high).
    Toggle,
}

// -----------------------------------------------------------------------------
// Set GPIO Pin Mode
// -----------------------------------------------------------------------------
/// Sets the mode (input/output/alternate/analog) for a specific GPIO pin.
///
/// # Arguments
/// * `port` - The base address of the GPIO port (e.g., `GPIOA_BASE`)
/// * `pin` - The pin number (0..15)
/// * `mode` - The desired mode as a `GpioMode` enum
///
/// This function modifies the MODER register for the selected pin.
pub fn gpio_set_mode(port: u32, pin: u32, mode: GpioMode) {
    let gpio_mode_reg_addr = (port + GPIOX_MODER) as *mut u32;
    let bit_position = pin * 2;
    let mode_value = mode as u32;

    // Set the 2 bits corresponding to the pin's mode in the MODER register
    reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

// -----------------------------------------------------------------------------
// Convenience Helpers: Set Pin Mode
// -----------------------------------------------------------------------------
/// Sets the specified GPIO pin to output mode.
///
/// # Arguments
/// * `port` - The base address of the GPIO port
/// * `pin` - The pin number (0..15)
pub fn gpio_set_mode_output(port: u32, pin: u32) {
    gpio_set_mode(port, pin, GpioMode::Output);
}

/// Sets the specified GPIO pin to input mode.
///
/// # Arguments
/// * `port` - The base address of the GPIO port
/// * `pin` - The pin number (0..15)
pub fn gpio_set_mode_input(port: u32, pin: u32) {
    gpio_set_mode(port, pin, GpioMode::Input);
}

/// Sets the specified GPIO pin to alternate function mode.
///
/// # Arguments
/// * `port` - The base address of the GPIO port
/// * `pin` - The pin number (0..15)
pub fn gpio_set_mode_alternate(port: u32, pin: u32) {
    gpio_set_mode(port, pin, GpioMode::Alternate);
}

/// Sets the specified GPIO pin to analog mode.
///
/// # Arguments
/// * `port` - The base address of the GPIO port
/// * `pin` - The pin number (0..15)
pub fn gpio_set_mode_analog(port: u32, pin: u32) {
    gpio_set_mode(port, pin, GpioMode::Analog);
}

// -----------------------------------------------------------------------------
// Set GPIO Output Type
// -----------------------------------------------------------------------------
/// Sets the output type (push-pull or open-drain) for a specific GPIO pin.
///
/// # Arguments
/// * `port` - The base address of the GPIO port
/// * `pin` - The pin number (0..15)
/// * `ty`  - The desired output type as a `GpioType` enum
///
/// This function modifies the OTYPER register for the selected pin.
pub fn gpio_set_type(port: u32, pin: u32, ty: GpioType) {
    let gpio_type_reg_addr = (port + GPIOX_OTYPER) as *mut u32;
    let bit_position = pin;
    let type_value = ty as u32;

    // Set the bit corresponding to the pin's output type in the OTYPER register
    reg_set_bits(gpio_type_reg_addr, type_value, bit_position, 1);
}

// -----------------------------------------------------------------------------
// Set GPIO Pin State
// -----------------------------------------------------------------------------
/// Sets the state of a GPIO pin (high, low, or toggle).
///
/// # Arguments
/// * `port` - The base address of the GPIO port.
/// * `pin` - The pin number (0-15).
/// * `pin_state` - The desired state (`PinState::High`, `PinState::Low`, or `PinState::Toggle`).
pub fn gpio_set_pin_state(port: u32, pin: u32, pin_state: PinState) {
    let gpio_bsrr_addr = (port + GPIOX_BSRR) as *mut u32;

    match pin_state {
        PinState::High => {
            // Set the pin high by writing to the BSRR register (lower 16 bits)
            reg_set_val(gpio_bsrr_addr, 1 << pin);
        }
        PinState::Low => {
            // Set the pin low by writing to the BSRR register (upper 16 bits)
            reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
        }
        PinState::Toggle => {
            // Read the current state and flip it
            let gpio_odr_addr = (port + GPIOX_ODR) as *mut u32;
            if reg_read_bit(gpio_odr_addr, pin) {
                // If currently high, set low
                reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
            } else {
                // If currently low, set high
                reg_set_val(gpio_bsrr_addr, 1 << pin);
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Read GPIO Pin State
// -----------------------------------------------------------------------------
/// Reads the current state of a GPIO pin.
///
/// # Arguments
/// * `port` - The base address of the GPIO port.
/// * `pin` - The pin number (0-15).
///
/// # Returns
/// * `true` if the pin is high, `false` if it is low.
pub fn gpio_get_pin_state(port: u32, pin: u32) -> bool {
    let gpio_idr_addr = (port + GPIOX_IDR) as *mut u32;
    reg_read_bit(gpio_idr_addr, pin)
}
