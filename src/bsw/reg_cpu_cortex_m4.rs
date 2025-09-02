/*!
Cortex-M4 core and CoreSight base addresses

This module defines memory-mapped register base addresses for ARMv7-M Cortex-M4
core components and CoreSight debug/trace blocks commonly present on STM32F429
and similar MCUs:

- SCB/System control (SysTick, CPUID, vector table, fault status, etc.)
- NVIC (interrupt enable/priority)
- MPU (memory protection unit)
- DWT (data watchpoint and trace, cycle counter)
- ITM (instrumentation trace macrocell)
- TPIU (trace port interface unit)
- Debug (DEMCR, DHCSR, etc.)

Notes
- These are base addresses as specified by the ARMv7-M Architecture Reference
  Manual (DDI 0403) and CoreSight specs. Some registers are arrays or have
  multiple words starting at these bases (see the ARM docs for field layouts).
- Access requires unsafe volatile reads/writes. Only interact with registers
  that are present and enabled on your device to avoid faults.
- ITM/TPIU/DWT often require enabling trace (DEMCR.TRCENA) before use.
- For STM32 devices, also consult the device reference manual.

Examples

Read CPUID:
```
unsafe {
    let cpuid = core::ptr::read_volatile(CPUID_BASE as *const u32);
    // use cpuid...
}
```

Configure and start SysTick (core clock, enable interrupt):
```
unsafe {
    // Reload value
    core::ptr::write_volatile(STRVR_BASE as *mut u32, 160000 - 1);
    // Clear current
    core::ptr::write_volatile(STCVR_BASE as *mut u32, 0);
    // ENABLE | TICKINT | CLKSOURCE (bits 0,1,2)
    core::ptr::write_volatile(STCSR_BASE as *mut u32, (1 << 0) | (1 << 1) | (1 << 2));
}
```

Enable DWT cycle counter:
```
unsafe {
    // Enable trace (DEMCR.TRCENA bit 24)
    let demcr = core::ptr::read_volatile(DEMCR_BASE as *const u32) | (1 << 24);
    core::ptr::write_volatile(DEMCR_BASE as *mut u32, demcr);

    // Reset counter
    core::ptr::write_volatile(DWT_CYCCNT_BASE as *mut u32, 0);

    // Enable CYCCNT (DWT_CTRL.CYCCNTENA bit 0)
    let dwt_ctrl = core::ptr::read_volatile(DWT_CTRL_BASE as *const u32) | (1 << 0);
    core::ptr::write_volatile(DWT_CTRL_BASE as *mut u32, dwt_ctrl);
}
```

References
- ARMv7‑M Architecture Reference Manual (DDI 0403)
- ARM Debug Interface and CoreSight Architecture (e.g., IHI 0029, IHI 0031)
- Your MCU vendor’s reference manual (e.g., ST RM0090 for STM32F429)
*/

//
// --------------------
// Cortex-M Memory-Mapped Registers
// --------------------
//

// System control registers
pub const ACTLR_BASE: u32 = 0xE000_E008; // Auxiliary Control Register, ACTLR
pub const STCSR_BASE: u32 = 0xE000_E010; // SysTick Control and Status Register
pub const STRVR_BASE: u32 = 0xE000_E014; // SysTick Reload Value Register
pub const STCVR_BASE: u32 = 0xE000_E018; // SysTick Current Value Register
pub const STCR_BASE: u32 = 0xE000_E01C; // SysTick Calibration Value Register
pub const CPUID_BASE: u32 = 0xE000_ED00; // Refer to the CPUID Base Register, CPUID
pub const ICSR_BASE: u32 = 0xE000_ED04; // Interrupt Control and State Register
pub const VTOR_BASE: u32 = 0xE000_ED08; // Vector Table Offset Register
pub const AIRCR_BASE: u32 = 0xE000_ED0C; // Application Interrupt and Reset Control Register.
pub const SCR_BASE: u32 = 0xE000_ED10; // System Control Register
pub const CCR_BASE: u32 = 0xE000_ED14; // Configuration and Control Register.
pub const SHPR1_BASE: u32 = 0xE000_ED18; // System Handler Priority Register 1
pub const SHPR2_BASE: u32 = 0xE000_ED1C; // System Handler Priority Register 2
pub const SHPR3_BASE: u32 = 0xE000_ED20; // System Handler Priority Register 3
pub const SHCSR_BASE: u32 = 0xE000_ED24; // System Handler Control and State Register
pub const CFSR_BASE: u32 = 0xE000_ED28; // Configurable Fault Status Registers
pub const HFSR_BASE: u32 = 0xE000_ED2C; // HardFault Status Register
pub const DFSR_BASE: u32 = 0xE000_ED30; // Debug Fault Status Register
pub const MMFAR_BASE: u32 = 0xE000_ED34; // MemManage Fault Address Register.
pub const BFAR_BASE: u32 = 0xE000_ED38; // BusFault Address Register.
pub const AFSR_BASE: u32 = 0xE000_ED3C; // See the Auxiliary Fault Status Register, AFSR
pub const ID_PFR0_BASE: u32 = 0xE000_ED40; // Processor Feature Register 0
pub const ID_PFR1_BASE: u32 = 0xE000_ED44; // Processor Feature Register 1
pub const ID_DFR0_BASE: u32 = 0xE000_ED48; // Debug Features Register 0.
pub const ID_AFR0_BASE: u32 = 0xE000_ED4C; // Auxiliary Features Register 0
pub const ID_MMFR0_BASE: u32 = 0xE000_ED50; // Memory Model Feature Register 0
pub const ID_MMFR1_BASE: u32 = 0xE000_ED54; // Memory Model Feature Register 1
pub const ID_MMFR2_BASE: u32 = 0xE000_ED58; // Memory Model Feature Register 2
pub const ID_MMFR3_BASE: u32 = 0xE000_ED5C; // Memory Model Feature Register 3
pub const ID_ISAR0_BASE: u32 = 0xE000_ED60; // Instruction Set Attributes Register 0
pub const ID_ISAR1_BASE: u32 = 0xE000_ED64; // Instruction Set Attributes Register 1
pub const ID_ISAR2_BASE: u32 = 0xE000_ED68; // Instruction Set Attributes Register 2
pub const ID_ISAR3_BASE: u32 = 0xE000_ED6C; // Instruction Set Attributes Register 3
pub const ID_ISAR4_BASE: u32 = 0xE000_ED70; // Instruction Set Attributes Register 4
pub const CPACR_BASE: u32 = 0xE000_ED88; // Coprocessor Access Control Register
pub const STIR_BASE: u32 = 0xE000_EF00; // Software Triggered Interrupt Register

// Memory Protection Unit (MPU)
pub const MPU_TYPE_BASE: u32 = 0xE000_ED90; // MPU Type Register
pub const MPU_CTRL_BASE: u32 = 0xE000_ED94; // MPU Control Register
pub const MPU_RNR_BASE: u32 = 0xE000_ED98; // MPU Region Number Register
pub const MPU_RBAR_BASE: u32 = 0xE000_ED9C; // MPU Region Base Address Register
pub const MPU_RASR_BASE: u32 = 0xE000_EDA0; // MPU Region Attribute and Size Register

// Nested Vectored Interrupt Controller (NVIC)
pub const NVIC_ICTR_BASE: u32 = 0xE000_E004; // Interrupt Controller Type Register
pub const NVIC_ISER_BASE: u32 = 0xE000_E100; // Interrupt Set-Enable Register
pub const NVIC_ICER_BASE: u32 = 0xE000_E180; // Interrupt Clear-Enable Register
pub const NVIC_ISPR_BASE: u32 = 0xE000_E200; // Interrupt Set-Pending Register
pub const NVIC_ICPR_BASE: u32 = 0xE000_E280; // Interrupt Clear-Pending Register
pub const NVIC_IABR_BASE: u32 = 0xE000_E300; // Interrupt Active Bit Register
pub const NVIC_IPR_BASE: u32 = 0xE000_E400; // Interrupt Priority Register

// Floating-Point (FP) and Media/VFP Registers
pub const FPCCR_BASE: u32 = 0xE000_EF34; // FP Context Control Register
pub const FPCAR_BASE: u32 = 0xE000_EF38; // FP Context Address Register
pub const FPDSCR_BASE: u32 = 0xE000_EF3C; // FP Default Status Control Register
pub const MVFR0_BASE: u32 = 0xE000_EF40; // Media and VFP Feature Register 0, MVFR0
pub const MVFR1_BASE: u32 = 0xE000_EF44; // Media and VFP Feature Register 1, MVFR1

// Debug Registers
//pub const DFSR_BASE: u32 = 0xE000_ED30; // Debug Fault Status Register. Power-on reset only.
pub const DHCSR_BASE: u32 = 0xE000_EDF0; // Debug Halting Control and Status Register
pub const DCRSR_BASE: u32 = 0xE000_EDF4; // Debug Core Register Selector Register
pub const DCRDR_BASE: u32 = 0xE000_EDF8; // Debug Core Register Data Register
pub const DEMCR_BASE: u32 = 0xE000_EDFC; // Debug Exception and Monitor Control Register

// DWT (Data Watchpoint and Trace) Registers
pub const DWT_CTRL_BASE: u32 = 0xE000_1000; // Control Register.
pub const DWT_CYCCNT_BASE: u32 = 0xE000_1004; // Cycle Count Register
pub const DWT_CPICNT_BASE: u32 = 0xE000_1008; // CPI Count Register
pub const DWT_EXCCNT_BASE: u32 = 0xE000_100C; // Exception Overhead Count Register
pub const DWT_SLEEPCNT_BASE: u32 = 0xE000_1010; // Sleep Count Register
pub const DWT_LSUCNT_BASE: u32 = 0xE000_1014; // LSU Count Register
pub const DWT_FOLDCNT_BASE: u32 = 0xE000_1018; // Folded-instruction Count Register
pub const DWT_PCSR_BASE: u32 = 0xE000_101C; // Program Counter Sample Register
pub const DWT_COMP0_BASE: u32 = 0xE000_1020; // Comparator Register0
pub const DWT_MASK0_BASE: u32 = 0xE000_1024; // Mask Register0.
pub const DWT_FUNCTION0_BASE: u32 = 0xE000_1028; // Function Register0
pub const DWT_COMP1_BASE: u32 = 0xE000_1030; // Comparator Register1
pub const DWT_MASK1_BASE: u32 = 0xE000_1034; // Mask Register1.
pub const DWT_FUNCTION1_BASE: u32 = 0xE000_1038; // Function Register1
pub const DWT_COMP2_BASE: u32 = 0xE000_1040; // Comparator Register2
pub const DWT_MASK2_BASE: u32 = 0xE000_1044; // Mask Register2.
pub const DWT_FUNCTION2_BASE: u32 = 0xE000_1048; // Function Register2
pub const DWT_COMP3_BASE: u32 = 0xE000_1050; // Comparator Register3
pub const DWT_MASK3_BASE: u32 = 0xE000_1054; // Mask Register3.
pub const DWT_FUNCTION3_BASE: u32 = 0xE000_1058; // Function Register3

// Peripheral and Component Identification Registers
pub const PID4_BASE: u32 = 0xE000_1FD0; // Peripheral identification register 4
pub const PID5_BASE: u32 = 0xE000_1FD4; // Peripheral identification register 5
pub const PID6_BASE: u32 = 0xE000_1FD8; // Peripheral identification register 6
pub const PID7_BASE: u32 = 0xE000_1FDC; // Peripheral identification register 7
pub const PID0_BASE: u32 = 0xE000_1FE0; // Peripheral identification register 0
pub const PID1_BASE: u32 = 0xE000_1FE4; // Peripheral identification register 1
pub const PID2_BASE: u32 = 0xE000_1FE8; // Peripheral identification register 2
pub const PID3_BASE: u32 = 0xE000_1FEC; // Peripheral identification register 3
pub const CID0_BASE: u32 = 0xE000_1FF0; // Component identification register 0
pub const CID1_BASE: u32 = 0xE000_1FF4; // Component identification register 1
pub const CID2_BASE: u32 = 0xE000_1FF8; // Component identification register 2
pub const CID3_BASE: u32 = 0xE000_1FFC; // Component identification register 3

// --------------------
// Instrumentation Trace Macrocell (ITM) Registers
// --------------------

// Stimulus Port Registers (0-31)
// The base address for the ITM Stimulus Port Registers, which are mapped from 0xE0000000 to 0xE000007C.
pub const ITM_STIM_BASE: u32 = 0xE000_0000;

// Trace Enable Register
pub const ITM_TER_BASE: u32 = 0xE000_0E00;

// Trace Privilege Register
pub const ITM_TPR_BASE: u32 = 0xE000_0E40;

// Trace Control Register
pub const ITM_TCR_BASE: u32 = 0xE000_0E80;

// Peripheral Identification registers
pub const ITM_PID4_BASE: u32 = 0xE000_0FD0;
pub const ITM_PID5_BASE: u32 = 0xE000_0FD4;
pub const ITM_PID6_BASE: u32 = 0xE000_0FD8;
pub const ITM_PID7_BASE: u32 = 0xE000_0FDC;
pub const ITM_PID0_BASE: u32 = 0xE000_0FE0;
pub const ITM_PID1_BASE: u32 = 0xE000_0FE4;
pub const ITM_PID2_BASE: u32 = 0xE000_0FE8;
pub const ITM_PID3_BASE: u32 = 0xE000_0FEC;

// Component Identification registers
pub const ITM_CID0_BASE: u32 = 0xE000_0FF0;
pub const ITM_CID1_BASE: u32 = 0xE000_0FF4;
pub const ITM_CID2_BASE: u32 = 0xE000_0FF8;
pub const ITM_CID3_BASE: u32 = 0xE000_0FFC;

// --------------------
// Trace Port Interface Unit (TPIU) Registers
// --------------------

// Supported Parallel Port Size Register
pub const TPIU_SSPSR_BASE: u32 = 0xE004_0000;

// Current Parallel Port Size Register
pub const TPIU_CSPSR_BASE: u32 = 0xE004_0004;

// Asynchronous Clock Prescaler Register
pub const TPIU_ACPR_BASE: u32 = 0xE004_0010;

// Selected Pin Protocol Register
pub const TPIU_SPPR_BASE: u32 = 0xE004_00F0;

// Formatter and Flush Status Register
pub const TPIU_FFSR_BASE: u32 = 0xE004_0300;

// Formatter and Flush Control Register
pub const TPIU_FFCR_BASE: u32 = 0xE004_0304;

// Formatter Synchronization Counter Register
pub const TPIU_FSCR_BASE: u32 = 0xE004_0308;

// TRIGGER register
pub const TPIU_TRIGGER_BASE: u32 = 0xE004_0EE8;

// Integration ETM Data
pub const TPIU_FIFO_DATA0_BASE: u32 = 0xE004_0EEC;

// ITATBCTR2
pub const TPIU_ITATBCTR2_BASE: u32 = 0xE004_0EF0;

// Integration ITM Data
pub const TPIU_FIFO_DATA1_BASE: u32 = 0xE004_0EFC;

// ITATBCTR0
pub const TPIU_ITATBCTR0_BASE: u32 = 0xE004_0EF8;

// Integration Mode Control
pub const TPIU_ITCTRL_BASE: u32 = 0xE004_0F00;

// Claim tag set
pub const TPIU_CLAIMSET_BASE: u32 = 0xE004_0FA0;

// Claim tag clear
pub const TPIU_CLAIMCLR_BASE: u32 = 0xE004_0FA4;

// TPIU_DEVID
pub const TPIU_DEVID_BASE: u32 = 0xE004_0FC8;

// TPIU_DEVTYPE: u32 = 0xE004_0FCC;

// Peripheral identification registers
pub const TPIU_PID4_BASE: u32 = 0xE004_0FD0;
pub const TPIU_PID5_BASE: u32 = 0xE004_0FD4;
pub const TPIU_PID6_BASE: u32 = 0xE004_0FD8;
pub const TPIU_PID7_BASE: u32 = 0xE004_0FDC;
pub const TPIU_PID0_BASE: u32 = 0xE004_0FE0;
pub const TPIU_PID1_BASE: u32 = 0xE004_0FE4;
pub const TPIU_PID2_BASE: u32 = 0xE004_0FE8;
pub const TPIU_PID3_BASE: u32 = 0xE004_0FEC;

// Component identification registers
pub const TPIU_CID0_BASE: u32 = 0xE004_0FF0;
pub const TPIU_CID1_BASE: u32 = 0xE004_0FF4;
pub const TPIU_CID2_BASE: u32 = 0xE004_0FF8;
pub const TPIU_CID3_BASE: u32 = 0xE004_0FFC;
