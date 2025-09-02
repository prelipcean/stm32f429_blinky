//! Basic Software (bsw) root module.
//!
//! Groups low-level CPU/MCU register access and startup code used by the application.
//! Target: STM32F429ZI (Cortex-M4F)

pub mod startup_stm32f429zi;
pub mod reg_utils;
pub mod reg_cpu_cortex_m4;
pub mod reg_mcu_stm32f429zi;
