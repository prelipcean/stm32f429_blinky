//! Custom startup code for STM32F429ZI.

#![allow(clippy::empty_loop)]

use core::ptr;

// Symbols provided by the linker script for memory initialization
unsafe extern "C" {
    static _sidata: u32; // Start of init values for .data
    static mut _sdata: u32; // Start of .data section
    static mut _edata: u32; // End of .data section
    static mut _sbss: u32; // Start of .bss section
    static mut _ebss: u32; // End of .bss section
}

// Interrupt vector table
#[used]
#[unsafe(link_section = ".isr_vector")]
static VECTOR_TABLE: [Option<extern "C" fn()>; 106] = [
    // Initial Stack Pointer (first entry, value provided by linker script)
    // This entry is filled by the linker script with _start_of_stack
    // Core Cortex-M Exceptions (Vector Table Position 1-15)
    Some(Reset_Handler),     // Position 1: Reset Handler (our custom entry point)
    Some(NMI_Handler),       // Position 2: Non-Maskable Interrupt
    Some(HardFault_Handler), // Position 3: Hard Fault
    Some(MemManage_Handler), // Position 4: Memory Management Fault
    Some(BusFault_Handler),  // Position 5: Bus Fault
    Some(UsageFault_Handler), // Position 6: Usage Fault
    None,
    None,
    None,
    None,                   // Position 7-10: Reserved
    Some(SVCall_Handler),   // Position 11: SVCall
    Some(DebugMon_Handler), // Position 12: Debug Monitor
    None,                   // Position 13: Reserved
    Some(PendSV_Handler),   // Position 14: PendSV
    Some(SysTick_Handler),  // Position 15: SysTick
    // STM32F429ZI Specific Peripheral Interrupts (IRQs) - starting from IRQ 0
    // These entries correspond to the IRQ numbers and their order in the STM32F429xx Reference Manual, Table 63.
    // Vector Table Position = 16 + IRQ Number
    Some(WWDG_Handler),               // IRQ0: Window Watchdog Interrupt
    Some(PVD_Handler),                // IRQ1: PVD through EXTI Line Detection Interrupt
    Some(TAMP_STAMP_Handler),         // IRQ2: Tamper and TimeStamp Interrupts
    Some(RTC_WKUP_Handler),           // IRQ3: RTC Wakeup Interrupt through the EXTI line
    Some(FLASH_Handler),              // IRQ4: FLASH global Interrupt
    Some(RCC_Handler),                // IRQ5: RCC global Interrupt
    Some(EXTI0_Handler),              // IRQ6: EXTI Line0 Interrupt
    Some(EXTI1_Handler),              // IRQ7: EXTI Line1 Interrupt
    Some(EXTI2_Handler),              // IRQ8: EXTI Line2 Interrupt
    Some(EXTI3_Handler),              // IRQ9: EXTI Line3 Interrupt
    Some(EXTI4_Handler),              // IRQ10: EXTI Line4 Interrupt
    Some(DMA1_Stream0_Handler),       // IRQ11: DMA1 Stream 0 global Interrupt
    Some(DMA1_Stream1_Handler),       // IRQ12: DMA1 Stream 1 global Interrupt
    Some(DMA1_Stream2_Handler),       // IRQ13: DMA1 Stream 2 global Interrupt
    Some(DMA1_Stream3_Handler),       // IRQ14: DMA1 Stream 3 global Interrupt
    Some(DMA1_Stream4_Handler),       // IRQ15: DMA1 Stream 4 global Interrupt
    Some(DMA1_Stream5_Handler),       // IRQ16: DMA1 Stream 5 global Interrupt
    Some(DMA1_Stream6_Handler),       // IRQ17: DMA1 Stream 6 global Interrupt
    Some(ADC_Handler),                // IRQ18: ADC1, ADC2 and ADC3 global Interrupts
    Some(CAN1_TX_Handler),            // IRQ19: CAN1 TX Interrupt
    Some(CAN1_RX0_Handler),           // IRQ20: CAN1 RX0 Interrupt
    Some(CAN1_RX1_Handler),           // IRQ21: CAN1 RX1 Interrupt
    Some(CAN1_SCE_Handler),           // IRQ22: CAN1 SCE Interrupt
    Some(EXTI9_5_Handler),            // IRQ23: EXTI Line[9:5] Interrupts
    Some(TIM1_BRK_TIM9_Handler),      // IRQ24: TIM1 Break interrupt and TIM9 global interrupt
    Some(TIM1_UP_TIM10_Handler),      // IRQ25: TIM1 Update Interrupt and TIM10 global interrupt
    Some(TIM1_TRG_COM_TIM11_Handler), // IRQ26: TIM1 Trigger and Commutation Interrupts and TIM11 global interrupt
    Some(TIM1_CC_Handler),            // IRQ27: TIM1 Capture Compare Interrupt
    Some(TIM2_Handler),               // IRQ28: TIM2 global Interrupt
    Some(TIM3_Handler),               // IRQ29: TIM3 global Interrupt
    Some(TIM4_Handler),               // IRQ30: TIM4 global Interrupt
    Some(I2C1_EV_Handler),            // IRQ31: I2C1 Event Interrupt
    Some(I2C1_ER_Handler),            // IRQ32: I2C1 Error Interrupt
    Some(I2C2_EV_Handler),            // IRQ33: I2C2 Event Interrupt
    Some(I2C2_ER_Handler),            // IRQ34: I2C2 Error Interrupt
    Some(SPI1_Handler),               // IRQ35: SPI1 global Interrupt
    Some(SPI2_Handler),               // IRQ36: SPI2 global Interrupt
    Some(USART1_Handler),             // IRQ37: USART1 global Interrupt
    Some(USART2_Handler),             // IRQ38: USART2 global Interrupt
    Some(USART3_Handler),             // IRQ39: USART3 global Interrupt
    Some(EXTI15_10_Handler),          // IRQ40: EXTI Line[15:10] Interrupts
    Some(RTC_Alarm_Handler),          // IRQ41: RTC Alarms (A and B) through EXTI Line Interrupt
    Some(OTG_FS_WKUP_Handler),        // IRQ42: USB OTG FS Wakeup through EXTI line interrupt
    Some(TIM8_BRK_TIM12_Handler),     // IRQ43: TIM8 Break Interrupt and TIM12 global interrupt
    Some(TIM8_UP_TIM13_Handler),      // IRQ44: TIM8 Update Interrupt and TIM13 global interrupt
    Some(TIM8_TRG_COM_TIM14_Handler), // IRQ45: TIM8 Trigger and Commutation Interrupts and TIM14 global interrupt
    Some(TIM8_CC_Handler),            // IRQ46: TIM8 Capture Compare Interrupt
    Some(DMA1_Stream7_Handler),       // IRQ47: DMA1 Stream7 global Interrupt
    Some(FSMC_Handler), // IRQ48: FSMC global Interrupt (Flexible Static Memory Controller)
    Some(SDIO_Handler), // IRQ49: SDIO global Interrupt
    Some(TIM5_Handler), // IRQ50: TIM5 global Interrupt
    Some(SPI3_Handler), // IRQ51: SPI3 global Interrupt
    Some(UART4_Handler), // IRQ52: UART4 global Interrupt
    Some(UART5_Handler), // IRQ53: UART5 global Interrupt
    Some(TIM6_DAC_Handler), // IRQ54: TIM6 global Interrupt and DAC1&2 underrun errors
    Some(TIM7_Handler), // IRQ55: TIM7 global Interrupt
    Some(DMA2_Stream0_Handler), // IRQ56: DMA2 Stream 0 global Interrupt
    Some(DMA2_Stream1_Handler), // IRQ57: DMA2 Stream 1 global Interrupt
    Some(DMA2_Stream2_Handler), // IRQ58: DMA2 Stream 2 global Interrupt
    Some(DMA2_Stream3_Handler), // IRQ59: DMA2 Stream 3 global Interrupt
    Some(DMA2_Stream4_Handler), // IRQ60: DMA2 Stream 4 global Interrupt
    Some(ETH_Handler),  // IRQ61: Ethernet global interrupt
    Some(ETH_WKUP_Handler), // IRQ62: Ethernet Wake-up through EXTI line interrupt
    Some(CAN2_TX_Handler), // IRQ63: CAN2 TX Interrupt
    Some(CAN2_RX0_Handler), // IRQ64: CAN2 RX0 Interrupt
    Some(CAN2_RX1_Handler), // IRQ65: CAN2 RX1 Interrupt
    Some(CAN2_SCE_Handler), // IRQ66: CAN2 SCE Interrupt
    Some(OTG_FS_Handler), // IRQ67: USB OTG FS global Interrupt
    Some(DMA2_Stream5_Handler), // IRQ68: DMA2 Stream 5 global Interrupt
    Some(DMA2_Stream6_Handler), // IRQ69: DMA2 Stream 6 global Interrupt
    Some(DMA2_Stream7_Handler), // IRQ70: DMA2 Stream 7 global Interrupt
    Some(USART6_Handler), // IRQ71: USART6 global Interrupt
    Some(I2C3_EV_Handler), // IRQ72: I2C3 Event Interrupt
    Some(I2C3_ER_Handler), // IRQ73: I2C3 Error Interrupt
    Some(OTG_HS_EP1_OUT_Handler), // IRQ74: USB OTG HS End Point 1 Out global Interrupt
    Some(OTG_HS_EP1_IN_Handler), // IRQ75: USB OTG HS End Point 1 In global Interrupt
    Some(OTG_HS_WKUP_Handler), // IRQ76: USB OTG HS Wakeup through EXTI line
    Some(OTG_HS_Handler), // IRQ77: USB OTG HS global Interrupt
    Some(DCMI_Handler), // IRQ78: DCMI global Interrupt
    Some(CRYP_Handler), // IRQ79: CRYP crypto global interrupt
    Some(HASH_RNG_Handler), // IRQ80: Hash and Rng global interrupt
    Some(FPU_Handler),  // IRQ81: FPU global Interrupt
    Some(UART7_Handler), // IRQ82: UART 7 global interrupt
    Some(UART8_Handler), // IRQ83: UART 8 global interrupt
    Some(SPI4_Handler), // IRQ84: SPI 4 global interrupt
    Some(SPI5_Handler), // IRQ85: SPI 5 global interrupt
    Some(SPI6_Handler), // IRQ86: SPI 6 global interrupt
    Some(SAI1_Handler), // IRQ87: SAI1 global interrupt
    Some(LCD_TFT_Handler), // IRQ88: LTDC global interrupt
    Some(LCD_TFT_Error_Handler), // IRQ89: LTDC global Error interrupt
    Some(DMA2D_Handler), // IRQ90: DMA2D global interrupt
];

// Default handler for unused interrupts
#[unsafe(no_mangle)]
extern "C" fn Default_Handler() {
    loop {}
}

// HardFault handler: traps the CPU in an infinite loop for debugging
#[unsafe(no_mangle)]
extern "C" fn HardFault_Handler() {
    loop {}
}

// NMI handler: traps the CPU in an infinite loop for debugging
#[unsafe(no_mangle)]
extern "C" fn NMI_Handler() {
    loop {}
}

// Reset handler: initializes memory and calls main
#[unsafe(no_mangle)]
extern "C" fn Reset_Handler() {
    unsafe {
        // Copy .data section from flash to RAM
        let mut src: *const u32 = ptr::addr_of!(_sidata);
        let mut dest: *mut u32 = ptr::addr_of_mut!(_sdata);
        let data_end: *mut u32 = ptr::addr_of_mut!(_edata);

        while dest < data_end {
            *dest = *src;
            dest = dest.add(1);
            src = src.add(1);
        }

        // Zero initialize the .bss section
        let mut bss_start: *mut u32 = ptr::addr_of_mut!(_sbss);
        let bss_end: *mut u32 = ptr::addr_of_mut!(_ebss);

        while bss_start < bss_end {
            *bss_start = 0;
            bss_start = bss_start.add(1);
        }

        // Enter main application
        crate::main();
    }
}

// Macro to define default handlers for all interrupts
#[macro_export]
macro_rules! default_handler {
    ($handler_name:ident) => {
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub extern "C" fn $handler_name() {
            loop {}
        }
    };
}

// Cortex-M system handlers
default_handler!(MemManage_Handler);
default_handler!(BusFault_Handler);
default_handler!(UsageFault_Handler);
default_handler!(SVCall_Handler);
default_handler!(DebugMon_Handler);
default_handler!(PendSV_Handler);
default_handler!(SysTick_Handler);

// Peripheral interrupt handlers (all default to infinite loop)
default_handler!(WWDG_Handler);
default_handler!(PVD_Handler);
default_handler!(TAMP_STAMP_Handler);
default_handler!(RTC_WKUP_Handler);
default_handler!(FLASH_Handler);
default_handler!(RCC_Handler);
default_handler!(EXTI0_Handler);
default_handler!(EXTI1_Handler);
default_handler!(EXTI2_Handler);
default_handler!(EXTI3_Handler);
default_handler!(EXTI4_Handler);
default_handler!(DMA1_Stream0_Handler);
default_handler!(DMA1_Stream1_Handler);
default_handler!(DMA1_Stream2_Handler);
default_handler!(DMA1_Stream3_Handler);
default_handler!(DMA1_Stream4_Handler);
default_handler!(DMA1_Stream5_Handler);
default_handler!(DMA1_Stream6_Handler);
default_handler!(DMA1_Stream7_Handler);
default_handler!(ADC_Handler);
default_handler!(CAN1_TX_Handler);
default_handler!(CAN1_RX0_Handler);
default_handler!(CAN1_RX1_Handler);
default_handler!(CAN1_SCE_Handler);
default_handler!(EXTI9_5_Handler);
default_handler!(TIM1_BRK_TIM9_Handler);
default_handler!(TIM1_UP_TIM10_Handler);
default_handler!(TIM1_TRG_COM_TIM11_Handler);
default_handler!(TIM1_CC_Handler);
default_handler!(TIM2_Handler);
default_handler!(TIM3_Handler);
default_handler!(TIM4_Handler);
default_handler!(I2C1_EV_Handler);
default_handler!(I2C1_ER_Handler);
default_handler!(I2C2_EV_Handler);
default_handler!(I2C2_ER_Handler);
default_handler!(SPI1_Handler);
default_handler!(SPI2_Handler);
default_handler!(USART1_Handler);
default_handler!(USART2_Handler);
default_handler!(USART3_Handler);
default_handler!(EXTI15_10_Handler);
default_handler!(RTC_Alarm_Handler);
default_handler!(OTG_FS_WKUP_Handler);
default_handler!(TIM8_BRK_TIM12_Handler);
default_handler!(TIM8_UP_TIM13_Handler);
default_handler!(TIM8_TRG_COM_TIM14_Handler);
default_handler!(TIM8_CC_Handler);
default_handler!(FSMC_Handler);
default_handler!(SDIO_Handler);
default_handler!(TIM5_Handler);
default_handler!(SPI3_Handler);
default_handler!(UART4_Handler);
default_handler!(UART5_Handler);
default_handler!(TIM6_DAC_Handler);
default_handler!(TIM7_Handler);
default_handler!(DMA2_Stream0_Handler);
default_handler!(DMA2_Stream1_Handler);
default_handler!(DMA2_Stream2_Handler);
default_handler!(DMA2_Stream3_Handler);
default_handler!(DMA2_Stream4_Handler);
default_handler!(ETH_Handler);
default_handler!(ETH_WKUP_Handler);
default_handler!(CAN2_TX_Handler);
default_handler!(CAN2_RX0_Handler);
default_handler!(CAN2_RX1_Handler);
default_handler!(CAN2_SCE_Handler);
default_handler!(OTG_FS_Handler);
default_handler!(DMA2_Stream5_Handler);
default_handler!(DMA2_Stream6_Handler);
default_handler!(DMA2_Stream7_Handler);
default_handler!(USART6_Handler);
default_handler!(I2C3_EV_Handler);
default_handler!(I2C3_ER_Handler);
default_handler!(OTG_HS_EP1_OUT_Handler);
default_handler!(OTG_HS_EP1_IN_Handler);
default_handler!(OTG_HS_WKUP_Handler);
default_handler!(OTG_HS_Handler);
default_handler!(DCMI_Handler);
default_handler!(CRYP_Handler);
default_handler!(HASH_RNG_Handler);
default_handler!(FPU_Handler);
default_handler!(UART7_Handler);
default_handler!(UART8_Handler);
default_handler!(SPI4_Handler);
default_handler!(SPI5_Handler);
default_handler!(SPI6_Handler);
default_handler!(SAI1_Handler);
default_handler!(LCD_TFT_Handler);
default_handler!(LCD_TFT_Error_Handler);
default_handler!(DMA2D_Handler);
