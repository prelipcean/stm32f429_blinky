/*
 * Linker script for STM32F429ZI microcontroller.
 *
 * This script defines the memory layout and how different sections of our
 * compiled program are placed into these memory regions.
 *
 */

/* Entry Point: The program starts at reset_handler */
ENTRY(reset_handler)

/* Define memory regions for STM32F429ZI */
MEMORY
{
  /* Flash memory: 2MB, used for executable code and read-only data (constants) */
  FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 2M

  /* SRAM1: 112KB, main RAM for data, stack, and heap */
  SRAM1 (rwx) : ORIGIN = 0x20000000, LENGTH = 112K

  /* SRAM2: 16KB, additional RAM */
  SRAM2 (rwx) : ORIGIN = 0x2001C000, LENGTH = 16K

  /* SRAM3: 64KB, additional RAM */
  SRAM3 (rwx) : ORIGIN = 0x20020000, LENGTH = 64K

  /* CCM (Core Coupled Memory) Data RAM: 64KB, faster RAM for critical data/code */
  CCMRAM (rwx) : ORIGIN = 0x10000000, LENGTH = 64K

  /* Backup SRAM: 4KB, battery-backed RAM for persistent data across resets */
  BKPSRAM (rw) : ORIGIN = 0x40024000, LENGTH = 4K
}

/* Define the initial stack pointer and minimum stack/heap sizes */
/* The stack grows downwards from the top of SRAM1 */
_start_of_stack = ORIGIN(SRAM1) + LENGTH(SRAM1) - 0x4; /* Stack starts at top of SRAM1 */

/* Minimum stack and heap sizes (adjust these values based on your application's needs) */
_min_stack_size = 0x400; /* 1KB minimum stack */
_min_heap_size  = 0x400; /* 1KB minimum heap */

/*
 * SECTIONS command tells the linker where to place each section in memory.
 * The output ELF file will have these sections at the specified addresses.
 */
SECTIONS
{
    /*
     * .text section:
     * - Contains all executable code and the interrupt vector table.
     * - Placed in FLASH memory.
     */
    .text :
    {
      . = ALIGN(4);               /* Align to 4 bytes */
      LONG(_start_of_stack);      /* Initial stack pointer value for vector table (first entry) */
      KEEP(*(.isr_vector));       /* Keep the interrupt vector table (provided by cortex-m-rt) */
      *(.text)                    /* Main code from our Rust application */
      *(.text.*)                  /* Any additional code sections (e.g., from dependencies) */
      . = ALIGN(4);
    } > FLASH

    /*
     * .rodata section:
     * - Contains read-only data (constants, string literals, lookup tables).
     * - Placed in FLASH memory, as it does not change during runtime.
     */
    .rodata :
    {
      . = ALIGN(4);
      *(.rodata)
      *(.rodata.*)
      . = ALIGN(4);
    } > FLASH

    /*
     * .data section:
     * - Contains initialized global and static variables.
     * - These variables are stored in FLASH with their initial values,
     * and then copied to SRAM1 at startup
     * - "AT> FLASH" means the initial values are loaded from Flash, but the section
     * itself resides and runs from SRAM1.
     */
    .data :
    {
      _sidata = LOADADDR(.data);  /* Load address: where .data is stored in FLASH */
      . = ALIGN(4);
      _sdata = .;                 /* Run address: start of .data in SRAM1 */
      *(.data)
      *(.data.*)
      . = ALIGN(4);
      _edata = .;                 /* End of .data in SRAM1 */
    } > SRAM1 AT> FLASH

    /*
     * .bss section:
     * - Contains uninitialized global and static variables.
     * - These variables are not stored in Flash; their memory space is reserved in SRAM1,
     * and they are zero-initialized by the startup code.
     * - Only exists in SRAM1 at runtime.
     */
    .bss :
    {
      . = ALIGN(4);
      _sbss = .;                  /* Start of .bss in SRAM1 */
      *(.bss)
      *(.bss.*)
      . = ALIGN(4);
      _ebss = .;                  /* End of .bss in SRAM1 */
    } > SRAM1

    /*
     * .ram_usage_check section:
     * - A special section to ensure that the combined size of .data, .bss,
     * stack, and heap does not exceed the available SRAM1.
     * - If it does, the linker will generate an error, preventing memory overflow.
     */
    .ram_usage_check :
    {
      . = ALIGN(8);
      . = . + _min_stack_size;    /* Reserve space for stack */
      . = . + _min_heap_size;     /* Reserve space for heap */
      . = ALIGN(8);
    } > SRAM1

    /*
     * .noinit section:
     * - Variables placed here will NOT be initialized or zeroed at startup.
     * - Useful for data that must persist across software resets (e.g., in Battery-backed SRAM).
     * - Placed in BKPSRAM (Backup SRAM).
     * - "NOLOAD" means this section is not loaded from Flash.
     * Example use in Rust: #[link_section = ".noinit"] static mut MY_VAR: u32 = 0;
     * Example use in C: __attribute__((section(".noinit"))) static unit32 my_var = 0u;
     */
    .noinit (NOLOAD) :
    {
      . = ALIGN(4);
      _snoinit = .;               /* Start of .noinit in BKPSRAM */
      *(.noinit)
      *(.noinit.*)
      . = ALIGN(4);
      _enoinit = .;               /* End of .noinit in BKPSRAM */
    } > BKPSRAM

    /*
     * .ccmram section:
     * - Variables specifically placed in the Core Coupled Memory (CCM) RAM.
     * - This RAM is typically faster and directly connected to the CPU,
     * ideal for performance-critical data.
     * - Placed in CCMRAM.
     * - "NOLOAD" means this section is not loaded from Flash.
     * Example use in Rust: #[link_section = ".ccmram"] static mut MY_CCM_VAR: u32 = 0;
     */
    .ccmram (NOLOAD) :
    {
      . = ALIGN(4);
      _sccmram = .;               /* Start of .ccmram in CCMRAM */
      *(.ccmram)
      *(.ccmram.*)
      . = ALIGN(4);
      _eccmram = .;               /* End of .ccmram in CCMRAM */
    } > CCMRAM
}
