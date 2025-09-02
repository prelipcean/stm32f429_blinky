use crate::bsw::gpio::{
    GPIO_PIN_13, GpioType, PinState, gpio_set_mode_output, gpio_set_pin_state, gpio_set_type,
};
use crate::bsw::rcc::rcc_enable_gpio_clock;
use crate::bsw::reg_mcu_stm32f429zi::*;

// Initialize the user GREEN LED GPIO pin ( PORT G, PIN 13 )
pub fn led_init() {
    // 1. Turn on the clock for the GPIO port
    rcc_enable_gpio_clock(GPIOG_BASE);

    // 2. Set the pin as an output (so you can control it)
    gpio_set_mode_output(GPIOG_BASE, GPIO_PIN_13);

    // 3. Make sure the pin is in push-pull mode (best for LEDs)
    gpio_set_type(GPIOG_BASE, GPIO_PIN_13, GpioType::PushPull);

    // 4. (Optional) Set the output speed if needed
}

pub fn led_on() {
    // Code to turn on the LED
    gpio_set_pin_state(GPIOG_BASE, GPIO_PIN_13, PinState::High);
}

pub fn led_off() {
    // Code to turn off the LED
    gpio_set_pin_state(GPIOG_BASE, GPIO_PIN_13, PinState::Low);
}

pub fn led_toggle() {
    // Code to toggle the LED state
    gpio_set_pin_state(GPIOG_BASE, GPIO_PIN_13, PinState::Toggle);
}
