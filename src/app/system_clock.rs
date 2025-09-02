use crate::bsw::flash::flash_set_wait_states;
use crate::bsw::gpio::{
    GPIO_PIN_8, PinSpeed, gpio_set_af, gpio_set_mode_alternate, gpio_set_speed,
};
use crate::bsw::pwr::*;
use crate::bsw::rcc::*;
use crate::bsw::reg_mcu_stm32f429zi::*;

pub fn system_clock_setup() {
    // Program flash wait states
    flash_set_wait_states(5);

    // Over drive settings
    rcc_enable_power_clock();
    pwr_set_regulator_voltage_scale(3);
    pwr_enable_overdrive();

    // Set PLL
    rcc_configure_pll_180mhz();
}

// Clock-out capability
pub fn system_clock_output_pa8() {
    // Turn on the clock for the GPIO port
    rcc_enable_gpio_clock(GPIOA_BASE);

    // Configure PA8 as alternate function (AF) for the clock output
    gpio_set_mode_alternate(GPIOA_BASE, GPIO_PIN_8);

    // Set output speed to very high
    gpio_set_speed(GPIOA_BASE, GPIO_PIN_8, PinSpeed::VeryHigh);

    // Set AF0 (MCO1) for PA8; see datasheet Table 12. STM32F427xx and STM32F429xx alternate function mapping
    gpio_set_af(GPIOA_BASE, GPIO_PIN_8, 0b0000);

    // Enable the clock output
    rcc_enable_mco1_output(McoSource::PLL, Div::Div4);
}
