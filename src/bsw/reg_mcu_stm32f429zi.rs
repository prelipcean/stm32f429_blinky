//! # STM32F429ZI Peripheral Base Addresses and IRQ Numbers
//!
//! This module provides raw memory-mapped base addresses for STM32F429ZI (and
//! compatible STM32F42x/43x family parts), grouped by bus (AHB1, AHB2, APB1, APB2),
//! plus the MCU external interrupt numbers (IRQn).
//!
//! Intended use:
//! - Bare-metal register access via unsafe volatile reads/writes.
//! - Minimal, dependency-free bring-up or quick experiments.
//! - Complement to Cortex‑M core register bases (see reg_cpu_cortex_m4.rs).
//!
//! Safety:
//! - Accessing these registers requires unsafe code and volatile operations
//!   (core::ptr::{read_volatile, write_volatile}).
//! - Only touch peripherals whose clocks are enabled in RCC to avoid bus faults.
//! - Verify your exact device variant; some peripherals may not be present.
//! - Consult the reference manual for register layouts and bit definitions.
//!
//! References:
//! - STM32F42x/43x Reference Manual (e.g., “RM0091” or device-appropriate RM).
//! - STM32F429x/439x Datasheet.
//! - ARMv7‑M Architecture Reference Manual for core peripherals.
//!
//! # Example Usage
//!
//! ## Toggling an LED on PD12
//!
//! Enable GPIOD clock and toggle PD12 (typical LED on many F4 boards):
//!
//! ```no_run
//! use core::ptr::{read_volatile, write_volatile};
//!
//! const RCC_AHB1ENR_OFFSET: usize = 0x30; // RCC AHB1 peripheral clock enable
//! const GPIOD_MODER_OFFSET:  usize = 0x00; // GPIO port mode register
//! const GPIOD_ODR_OFFSET:    usize = 0x14; // GPIO port output data register
//!
//! unsafe {
//!     // 1) Enable GPIOD clock
//!     let rcc_ahb1enr = (crate::mcu::reg_mcu_stm32f429zi::RCC_BASE as *mut u32)
//!         .add(RCC_AHB1ENR_OFFSET / 4);
//!     let mut v = read_volatile(rcc_ahb1enr);
//!     v |= 1 << 3; // GPIODEN
//!     write_volatile(rcc_ahb1enr, v);
//!
//!     // 2) Set PD12 to output mode (MODER12 = 0b01)
//!     let gpiod_moder = (crate::mcu::reg_mcu_stm32f429zi::GPIOD_BASE as *mut u32)
//!         .add(GPIOD_MODER_OFFSET / 4);
//!     let mut moder = read_volatile(gpiod_moder);
//!     moder &= !(0b11 << (12 * 2));
//!     moder |=  (0b01 << (12 * 2));
//!     write_volatile(gpiod_moder, moder);
//!
//!     // 3) Toggle PD12
//!     let gpiod_odr = (crate::mcu::reg_mcu_stm32f429zi::GPIOD_BASE as *mut u32)
//!         .add(GPIOD_ODR_OFFSET / 4);
//!     let odr = read_volatile(gpiod_odr) ^ (1 << 12);
//!     write_volatile(gpiod_odr, odr);
//! }
//! ```
//!
//! ## Enabling an Interrupt in NVIC
//!
//! Example: enabling an interrupt in NVIC (pair with core NVIC bases)
//! - Use NVIC_ISERx at 0xE000_E100 + (irqn / 32) * 4 (see reg_cpu_cortex_m4.rs).
//! - Set bit (irqn % 32) to enable the given interrupt.
//! - Refer to the device’s vector table for handler placement.

// --------------------
// AHB1 Peripherals (General-purpose I/O, DMA, etc.)
// --------------------
pub const GPIOA_BASE: u32 = 0x4002_0000; // GPIOA base address
pub const GPIOB_BASE: u32 = 0x4002_0400; // GPIOB base address
pub const GPIOC_BASE: u32 = 0x4002_0800; // GPIOC base address
pub const GPIOD_BASE: u32 = 0x4002_0C00; // GPIOD base address
pub const GPIOE_BASE: u32 = 0x4002_1000; // GPIOE base address
pub const GPIOF_BASE: u32 = 0x4002_1400; // GPIOF base address
pub const GPIOG_BASE: u32 = 0x4002_1800; // GPIOG base address
pub const GPIOH_BASE: u32 = 0x4002_1C00; // GPIOH base address
pub const GPIOI_BASE: u32 = 0x4002_2000; // GPIOI base address
pub const GPIOJ_BASE: u32 = 0x4002_2400; // GPIOJ base address
pub const GPIOK_BASE: u32 = 0x4002_2800; // GPIOK base address

pub const BKPSRAM_BASE: u32 = 0x4002_4000; // Backup SRAM
pub const DMA1_BASE: u32 = 0x4002_6000; // DMA1
pub const DMA2_BASE: u32 = 0x4002_6400; // DMA2
pub const ETH_BASE: u32 = 0x4002_8000; // Ethernet MAC (start)
pub const CRC_BASE: u32 = 0x4002_3000; // CRC base address
pub const RCC_BASE: u32 = 0x4002_3800; // RCC base address
pub const FLASH_INTERFACE_BASE: u32 = 0x4002_3C00; // Flash interface register

pub const USB_OTG_HS_BASE: u32 = 0x4004_0000; // USB OTG HS
pub const DMA2D_BASE: u32 = 0x4002_B000; // DMA2D
pub const ETH_MAC_BASE: u32 = 0x4002_8000; // Ethernet MAC

// --------------------
// APB2 Peripherals (High-speed peripherals)
// --------------------
pub const SYSCFG_BASE: u32 = 0x4001_3800; // SYSCFG base address
pub const EXTI_BASE: u32 = 0x4001_3C00; // EXTI base address
pub const SPI1_BASE: u32 = 0x4001_3000; // SPI1
pub const SPI4_BASE: u32 = 0x4001_3400; // SPI4
pub const TIM1_BASE: u32 = 0x4001_0000; // TIM1
pub const TIM8_BASE: u32 = 0x4001_0400; // TIM8
pub const USART1_BASE: u32 = 0x4001_1000; // USART1
pub const USART6_BASE: u32 = 0x4001_1400; // USART6
pub const ADC1_BASE: u32 = 0x4001_2000; // ADC1-3 shared base

pub const LCD_TFT_BASE: u32 = 0x4001_6800; // LCD-TFT
pub const SAI1_BASE: u32 = 0x4001_5800; // SAI1
pub const SPI6_BASE: u32 = 0x4001_5400; // SPI6
pub const SPI5_BASE: u32 = 0x4001_5000; // SPI5
pub const TIM11_BASE: u32 = 0x4001_4800; // TIM11
pub const TIM10_BASE: u32 = 0x4001_4400; // TIM10
pub const TIM9_BASE: u32 = 0x4001_4000; // TIM9
pub const SDIO_BASE: u32 = 0x4001_2C00; // SDIO

// --------------------
// APB1 Peripherals (Low-speed peripherals)
// --------------------
pub const TIM2_BASE: u32 = 0x4000_0000; // TIM2
pub const TIM3_BASE: u32 = 0x4000_0400; // TIM3
pub const TIM4_BASE: u32 = 0x4000_0800; // TIM4
pub const TIM5_BASE: u32 = 0x4000_0C00; // TIM5
pub const TIM6_BASE: u32 = 0x4000_1000; // TIM6
pub const TIM7_BASE: u32 = 0x4000_1400; // TIM7
pub const TIM12_BASE: u32 = 0x4000_1800; // TIM12
pub const TIM13_BASE: u32 = 0x4000_1C00; // TIM13
pub const TIM14_BASE: u32 = 0x4000_2000; // TIM14
pub const WWDG_BASE: u32 = 0x4000_2C00; // WWDG (Window Watchdog Timer)
pub const IWDG_BASE: u32 = 0x4000_3000; // IWDG (Independent Watchdog Timer)
pub const SPI2_BASE: u32 = 0x4000_3800; // SPI2/I2S2
pub const SPI3_BASE: u32 = 0x4000_3C00; // SPI3/I2S3
pub const I2S3EXT_BASE: u32 = 0x4000_4000; // I2S3ext
pub const I2S2EXT_BASE: u32 = 0x4000_3400; // I2S2ext
pub const USART2_BASE: u32 = 0x4000_4400; // USART2
pub const USART3_BASE: u32 = 0x4000_4800; // USART3
pub const UART4_BASE: u32 = 0x4000_4C00; // UART4
pub const UART5_BASE: u32 = 0x4000_5000; // UART5
pub const I2C1_BASE: u32 = 0x4000_5400; // I2C1
pub const I2C2_BASE: u32 = 0x4000_5800; // I2C2
pub const I2C3_BASE: u32 = 0x4000_5C00; // I2C3
pub const CAN1_BASE: u32 = 0x4000_6400; // CAN1
pub const CAN2_BASE: u32 = 0x4000_6800; // CAN2
pub const PWR_BASE: u32 = 0x4000_7000; // PWR (Power control)
pub const DAC_BASE: u32 = 0x4000_7400; // DAC (Digital-to-Analog Converter)
pub const UART7_BASE: u32 = 0x4000_7800; // UART7
pub const UART8_BASE: u32 = 0x4000_7C00; // UART8
pub const RTC_BKP_BASE: u32 = 0x4000_2800; // RTC & Backup Registers

// --------------------
// AHB2 Peripherals
// --------------------
pub const USB_OTG_FS_BASE: u32 = 0x5000_0000; // USB OTG FS
pub const DCMI_BASE: u32 = 0x5005_0000; // DCMI (Digital Camera Interface)
pub const RNG_BASE: u32 = 0x5006_0800; // RNG (Random Number Generator)

// --------------------
// Cortex-M4 Internal Peripherals
// --------------------
pub const CORTEX_M4_PERIPH_BASE: u32 = 0xE000_0000; // Start of Cortex-M4 internal peripherals

// --------------------
// FMC (Flexible Memory Controller) Banks
// --------------------
pub const FMC_BANK1_BASE: u32 = 0x6000_0000;
pub const FMC_BANK2_BASE: u32 = 0x7000_0000;
pub const FMC_BANK3_BASE: u32 = 0x8000_0000;
pub const FMC_BANK4_BASE: u32 = 0x9000_0000;
pub const FMC_CTRL_BASE: u32 = 0xA000_0000; // FMC control register
pub const FMC_BANK5_BASE: u32 = 0xC000_0000;
pub const FMC_BANK6_BASE: u32 = 0xD000_0000;

// --------------------
// Reserved/Boundary Addresses (for documentation)
// --------------------
pub const RESERVED_E00F_FFFF: u32 = 0xE00F_FFFF; // End of Cortex-M4 internal peripherals
pub const RESERVED_FFFF_FFFF: u32 = 0xFFFF_FFFF; // End of address space

/// Interrupt Request Numbers (IRQn) for STM32F4 external interrupts.
/// These values map to the NVIC external interrupt lines (not core exceptions).
/// Enable/disable via NVIC_ISERx/ICERx at the Cortex-M NVIC base (see core module).
#[allow(non_camel_case_types)]
pub enum IRQn {
    WWDG = 0,                // WWDG Window Watchdog interrupt; Address: 0x0000 0040
    PVD = 1,                 // PVD PVD through EXTI line detection interrupt; Address: 0x0000 0044
    TAMP_STAMP = 2, // TAMP_STAMP Tamper and TimeStamp interrupts through the EXTI line; Address: 0x0000 0048
    RTC_WKUP = 3,   // RTC_WKUP RTC Wake-up interrupt through the EXTI line; Address: 0x0000 004C
    FLASH = 4,      // FLASH Flash global interrupt; Address: 0x0000 0050
    RCC = 5,        // RCC RCC global interrupt; Address: 0x0000 0054
    EXTI0 = 6,      // EXTI0 EXTI Line0 interrupt; Address: 0x0000 0058
    EXTI1 = 7,      // EXTI1 EXTI Line1 interrupt; Address: 0x0000 005C
    EXTI2 = 8,      // EXTI2 EXTI Line2 interrupt; Address: 0x0000 0060
    EXTI3 = 9,      // EXTI3 EXTI Line3 interrupt; Address: 0x0000 0064
    EXTI4 = 10,     // EXTI4 EXTI Line4 interrupt; Address: 0x0000 0068
    DMA1_Stream0 = 11, // DMA1_Stream0 DMA1 Stream0 global interrupt; Address: 0x0000 006C
    DMA1_Stream1 = 12, // DMA1_Stream1 DMA1 Stream1 global interrupt; Address: 0x0000 0070
    DMA1_Stream2 = 13, // DMA1_Stream2 DMA1 Stream2 global interrupt; Address: 0x0000 0074
    DMA1_Stream3 = 14, // DMA1_Stream3 DMA1 Stream3 global interrupt; Address: 0x0000 0078
    DMA1_Stream4 = 15, // DMA1_Stream4 DMA1 Stream4 global interrupt; Address: 0x0000 007C
    DMA1_Stream5 = 16, // DMA1_Stream5 DMA1 Stream5 global interrupt; Address: 0x0000 0080
    DMA1_Stream6 = 17, // DMA1_Stream6 DMA1 Stream6 global interrupt; Address: 0x0000 0084
    ADC = 18,       // ADC ADC1, ADC2 and ADC3 global interrupts; Address: 0x0000 0088
    CAN1_TX = 19,   // CAN1_TX CAN1 TX interrupts; Address: 0x0000 008C
    CAN1_RX0 = 20,  // CAN1_RX0 CAN1 RX0 interrupts; Address: 0x0000 0090
    CAN1_RX1 = 21,  // CAN1_RX1 CAN1 RX1 interrupt; Address: 0x0000 0094
    CAN1_SCE = 22,  // CAN1_SCE CAN1 SCE interrupt; Address: 0x0000 0098
    EXTI9_5 = 23,   // EXTI9_5 EXTI Line[9:5] interrupts; Address: 0x0000 009C
    TIM1_BRK_TIM9 = 24, // TIM1_BRK_TIM9 TIM1 Break interrupt and TIM9 global interrupt; Address: 0x0000 00A0
    TIM1_UP_TIM10 = 25, // TIM1_UP_TIM10 TIM1 Update interrupt and TIM10 global interrupt; Address: 0x0000 00A4
    TIM1_TRG_COM_TIM11 = 26, // TIM1_TRG_COM_TIM11 TIM1 Trigger and Commutation interrupts and TIM11 global interrupt; Address: 0x0000 00A8
    TIM1_CC = 27,            // TIM1_CC TIM1 Capture Compare interrupt; Address: 0x0000 00AC
    TIM2 = 28,               // TIM2 TIM2 global interrupt; Address: 0x0000 00B0
    TIM3 = 29,               // TIM3 TIM3 global interrupt; Address: 0x0000 00B4
    TIM4 = 30,               // TIM4 TIM4 global interrupt; Address: 0x0000 00B8
    I2C1_EV = 31,            // I2C1_EV I2C1 event interrupt; Address: 0x0000 00BC
    I2C1_ER = 32,            // I2C1_ER I2C1 error interrupt; Address: 0x0000 00C0
    I2C2_EV = 33,            // I2C2_EV I2C2 event interrupt; Address: 0x0000 00C4
    I2C2_ER = 34,            // I2C2_ER I2C2 error interrupt; Address: 0x0000 00C8
    SPI1 = 35,               // SPI1 SPI1 global interrupt; Address: 0x0000 00CC
    SPI2 = 36,               // SPI2 SPI2 global interrupt; Address: 0x0000 00D0
    USART1 = 37,             // USART1 USART1 global interrupt; Address: 0x0000 00D4
    USART2 = 38,             // USART2 USART2 global interrupt; Address: 0x0000 00D8
    USART3 = 39,             // USART3 USART3 global interrupt; Address: 0x0000 00DC
    EXTI15_10 = 40,          // EXTI15_10 EXTI Line[15:10] interrupts; Address: 0x0000 00E0
    RTC_Alarm = 41, // RTC_Alarm RTC Alarms (A and B) through EXTI line interrupt; Address: 0x0000 00E4
    OTG_FS_WKUP = 42, // OTG_FS_WKUP USB On-The-Go FS Wake-up through EXTI line interrupt; Address: 0x0000 00E8
    TIM8_BRK_TIM12 = 43, // TIM8_BRK_TIM12 TIM8 Break interrupt and TIM12 global interrupt; Address: 0x0000 00EC
    TIM8_UP_TIM13 = 44,  // TIM8_UP_TIM13 TIM8 Update interrupt
    TIM8_TRG_COM_TIM14 = 45, // TIM8_TRG_COM_TIM14 TIM8 Trigger and Commutation interrupts and TIM14 global interrupt; Address: 0x0000 00F4
    TIM8_CC = 46,            // TIM8_CC TIM8 Capture Compare interrupt; Address: 0x0000 00F8
    DMA1_Stream7 = 47,       // DMA1_Stream7 DMA1 Stream7 global interrupt; Address: 0x0000 00FC
    FSMC = 48,               // FSMC FSMC global interrupt; Address: 0x0000 0100
    SDIO = 49,               // SDIO SDIO global interrupt; Address: 0x0000 0104
    TIM5 = 50,               // TIM5 TIM5 global interrupt; Address: 0x0000 0108
    SPI3 = 51,               // SPI3 SPI3 global interrupt; Address: 0x0000 010C
    UART4 = 52,              // UART4 UART4 global interrupt; Address: 0x0000 0110
    UART5 = 53,              // UART5 UART5 global interrupt; Address: 0x0000 0114
    TIM6_DAC = 54, // TIM6_DAC TIM6 global interrupt, DAC1 and DAC2 underrun errorinterrupts; Address: 0x0000 0118
    TIM7 = 55,     // TIM7 TIM7 global interrupt; Address: 0x0000 011C
    DMA2_Stream0 = 56, // DMA2_Stream0 DMA2 Stream0 global interrupt; Address: 0x0000 0120
    DMA2_Stream1 = 57, // DMA2_Stream1 DMA2 Stream1 global interrupt; Address: 0x0000 0124
    DMA2_Stream2 = 58, // DMA2_Stream2 DMA2 Stream2 global interrupt; Address: 0x0000 0128
    DMA2_Stream3 = 59, // DMA2_Stream3 DMA2 Stream3 global interrupt; Address: 0x0000 012C
    DMA2_Stream4 = 60, // DMA2_Stream4 DMA2 Stream4 global interrupt; Address: 0x0000 0130
    ETH = 61,      // ETH Ethernet global interrupt; Address: 0x0000 0134
    ETH_WKUP = 62, // ETH_WKUP Ethernet Wake-up through EXTI lineinterrupt; Address: 0x0000 0138
    CAN2_TX = 63,  // CAN2_TX CAN2 TX interrupts; Address: 0x0000 013C
    CAN2_RX0 = 64, // CAN2_RX0 CAN2 RX0 interrupts; Address: 0x0000 0140
    CAN2_RX1 = 65, // CAN2_RX1 CAN2 RX1 interrupt; Address: 0x0000 0144
    CAN2_SCE = 66, // CAN2_SCE CAN2 SCE interrupt; Address: 0x0000 0148
    OTG_FS = 67,   // OTG_FS USB On The Go FS global interrupt; Address: 0x0000 014C
    DMA2_Stream5 = 68, // DMA2_Stream5 DMA2 Stream5 global interrupt; Address: 0x0000 0150
    DMA2_Stream6 = 69, // DMA2_Stream6 DMA2 Stream6 global interrupt; Address: 0x0000 0154
    DMA2_Stream7 = 70, // DMA2_Stream7 DMA2 Stream7 global interrupt; Address: 0x0000 0158
    USART6 = 71,   // USART6 USART6 global interrupt; Address: 0x0000 015C
    I2C3_EV = 72,  // I2C3_EV I2C3 event interrupt; Address: 0x0000 0160
    I2C3_ER = 73,  // I2C3_ER I2C3 error interrupt; Address: 0x0000 0164
    OTG_HS_EP1_OUT = 74, // OTG_HS_EP1_OUT USB On The Go HS End Point 1 Out global interrupt; Address: 0x0000 0168
    OTG_HS_EP1_IN = 75, // OTG_HS_EP1_IN USB On The Go HS End Point 1 In global interrupt; Address: 0x0000 016C
    OTG_HS_WKUP = 76, // OTG_HS_WKUP USB On The Go HS Wake-up through EXTI interrupt; Address: 0x0000 0170
    OTG_HS = 77,      // OTG_HS USB On The Go HS global interrupt; Address: 0x0000 0174
    DCMI = 78,        // DCMI DCMI global interrupt; Address: 0x0000 0178
    CRYP = 79,        // CRYP CRYP crypto global interrupt; Address: 0x0000 017C
    HASH_RNG = 80,    // HASH_RNG Hash and Rng global interrupt; Address: 0x0000 0180
    FPU = 81,         // FPU FPU global interrupt; Address: 0x0000 0184
}
