// -----------------------------------------------------------------------------
// STM32F429 CAN (Controller Area Network) utilities
// -----------------------------------------------------------------------------
// Minimal helpers for CAN1/CAN2: clock init, GPIO AF config, filter setup, TX/RX.
// Uses raw MMIO (direct memory-mapped I/O); enable RCC APB1 clocks and configure GPIO pins before use.
