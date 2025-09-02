//! Register helper functions
//!
//! These helpers let you safely read and write memory‑mapped hardware registers (32‑bit).
//!
//! You can:
//! - Read or write a whole 32‑bit register
//! - Turn a single bit on or off
//! - Read or change a group of bits (bit fields)
//!
//! Important: Only use real hardware register addresses. Using a bad address can crash or freeze the MCU.
//! 
//! API overview
//! - reg_assert_mask_fits
//! - reg_read
//! - reg_write
//! - reg_set_bits
//! - reg_set_bit
//! - reg_set_val
//! - reg_read_bit
//! - reg_read_bits
//! - reg_clr_bit
//! - reg_set_bit_high
//! - reg_clr_val
//! - reg_set_val_masked
//! - reg_read_val_masked
//! - reg_toggle_bit
//! - reg_toggle_bits
//! - reg_modify
//! - reg_wait_bit
//! - reg_wait_bits
//! - reg_test_and_set
//! - reg_test_and_clear
//! - reg_count_set_bits
//! - reg_find_first_set
//!
//! In‑place (RAM) helpers
//! - reg_write_inplace
//! - reg_read_inplace
//! - reg_set_bit_inplace
//! - reg_clr_bit_inplace
//! - reg_toggle_bit_inplace
//! - reg_read_bit_inplace
//! - reg_clr_val_inplace
//! - reg_set_val_inplace
//! - reg_read_val_inplace

use core::ptr;
use core::hint;

/// A type alias for a hardware register address (pointer to a 32‑bit register).
/// Makes the intent of pointers clearer in code.
pub type RegisterAddress = *mut u32;

/// Common bit mask helpers
pub mod bit_masks {
    /// Create a mask with `n` consecutive 1 bits (from bit 0).
    /// n=0 -> 0, n=32 -> 0xFFFF_FFFF
    pub const fn mask_n_bits(n: u32) -> u32 {
        if n >= 32 { 0xFFFF_FFFF } else { (1u32 << n) - 1 }
    }
    
    /// Create a mask with a single 1 at `position`.
    /// position >= 32 -> 0
    pub const fn single_bit(position: u32) -> u32 {
        if position >= 32 { 0 } else { 1u32 << position }
    }
    
    /// Predefined 4‑bit masks (nibbles)
    pub const NIBBLE_0: u32 = 0x0000000F;
    pub const NIBBLE_1: u32 = 0x000000F0;
    pub const NIBBLE_2: u32 = 0x00000F00;
    pub const NIBBLE_3: u32 = 0x0000F000;
    pub const NIBBLE_4: u32 = 0x000F0000;
    pub const NIBBLE_5: u32 = 0x00F00000;
    pub const NIBBLE_6: u32 = 0x0F000000;
    pub const NIBBLE_7: u32 = 0xF0000000;
    
    /// Predefined byte masks
    pub const BYTE_0: u32 = 0x000000FF;
    pub const BYTE_1: u32 = 0x0000FF00;
    pub const BYTE_2: u32 = 0x00FF0000;
    pub const BYTE_3: u32 = 0xFF000000;
    
    /// Predefined half‑word masks (16 bits)
    pub const HALF_WORD_0: u32 = 0x0000FFFF;
    pub const HALF_WORD_1: u32 = 0xFFFF0000;
}

/// Verifies that shifting `mask` left by `bit_position` will still fit in a 32‑bit register.
/// - If `mask` is zero, there’s nothing to place and the check is skipped.
/// - Otherwise, it finds the highest set bit in `mask` and asserts that
///   `bit_position + highest_bit + 1` does not exceed 32 (bits 0..31).
#[inline(always)]
fn reg_assert_mask_fits(mask: u32, bit_position: u32) {
    if mask != 0 {
        let highest = 31 - mask.leading_zeros();
        assert!(
            (bit_position + highest) < 32,
            "mask << bit_position exceeds 32-bit register width"
        );
    }
}

/// Read a 32‑bit value from a memory‑mapped register.
///
/// Safety
/// - Unsafe because it dereferences a raw pointer.
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// let value = unsafe { reg_read(0x4800_0000 as RegisterAddress) };
/// ```
#[inline(always)]
pub unsafe fn reg_read(addr: RegisterAddress) -> u32 {
    debug_assert!((addr as usize & 0x3) == 0, "unaligned register address");
    unsafe { ptr::read_volatile(addr) }
}

/// Write a 32‑bit value to a memory‑mapped register.
///
/// Safety
/// - Unsafe because it dereferences a raw pointer.
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// unsafe { reg_write(0x4800_0000 as RegisterAddress, 0x1); }
/// ```
#[inline(always)]
pub unsafe fn reg_write(addr: RegisterAddress, value: u32) {
    debug_assert!((addr as usize & 0x3) == 0, "unaligned register address");
    unsafe { ptr::write_volatile(addr, value) }
}

/// Set a group of bits (bit field) in a register without touching other bits.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `new_bits_val`: New value for the field (must fit in `n_bits`)
/// - `bit_position`: Starting bit position (0 = least significant bit)
/// - `n_bits`: Number of bits in the field (1..=32)
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// // Set bits 4..=5 to binary 10 (decimal 2)
/// reg_set_bits(0x4800_0000 as RegisterAddress, 0b10, 4, 2);
/// ```
pub fn reg_set_bits(reg_addr: RegisterAddress, new_bits_val: u32, bit_position: u32, n_bits: u32) {
    assert!(n_bits > 0 && n_bits <= 32, "n_bits must be between 1 and 32");
    assert!(bit_position < 32, "bit_position must be less than 32");
    assert!(bit_position + n_bits <= 32, "bit range exceeds register size");

    // Ensure the provided value fits in the number of bits requested.
    let field_mask = bit_masks::mask_n_bits(n_bits);
    assert!(
        (new_bits_val & !field_mask) == 0,
        "new_bits_val does not fit in n_bits"
    );

    unsafe {
        let reg_value = reg_read(reg_addr);
        let mask = (((1u64 << n_bits) - 1) as u32) << bit_position;
        let updated_value = (reg_value & !mask) | ((new_bits_val << bit_position) & mask);
        reg_write(reg_addr, updated_value);
    }
}

/// Set or clear a single bit in a register (turn it on/off).
///
/// Arguments
/// - `reg_addr`: Register address
/// - `bit_position`: Bit index (0..31)
/// - `bit_val`: true = set to 1, false = set to 0
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// // Set bit 3 to 1
/// reg_set_bit(0x4800_0000 as RegisterAddress, 3, true);
/// ```
pub fn reg_set_bit(reg_addr: RegisterAddress, bit_position: u32, bit_val: bool) {
    assert!(bit_position < 32, "bit_position must be less than 32");
    
    unsafe {
        let reg_value = reg_read(reg_addr);
        let updated_value = if bit_val {
            reg_value | (1u32 << bit_position)
        } else {
            reg_value & !(1u32 << bit_position)
        };
        reg_write(reg_addr, updated_value);
    }
}

/// Write a new value to the entire 32‑bit register (replace all bits).
///
/// Arguments
/// - `reg_addr`: Register address
/// - `new_reg_val`: New 32‑bit value
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// reg_set_val(0x4800_0000 as RegisterAddress, 0xFFFF);
/// ```
pub fn reg_set_val(reg_addr: RegisterAddress, new_reg_val: u32) {
    unsafe {
        reg_write(reg_addr, new_reg_val);
    }
}

/// Read a single bit from a register.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `bit_position`: Bit index (0..31)
///
/// Returns
/// - true if the bit is 1, false if it is 0
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// let is_set = reg_read_bit(0x4800_0000 as RegisterAddress, 7);
/// ```
pub fn reg_read_bit(reg_addr: RegisterAddress, bit_position: u32) -> bool {
    assert!(bit_position < 32, "bit_position must be less than 32");
    
    unsafe {
        let reg_value = reg_read(reg_addr);
        (reg_value & (1u32 << bit_position)) != 0
    }
}

/// Read `n_bits` starting at `bit_position`. The result is right‑aligned.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `bit_position`: Starting bit position (0..31)
/// - `n_bits`: Number of bits to read (1..=32)
///
/// Returns
/// - The selected bits, shifted down so they start at bit 0
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// // Read 4 bits starting at position 8
/// let value = reg_read_bits(0x4800_0000 as RegisterAddress, 8, 4);
/// ```
pub fn reg_read_bits(reg_addr: RegisterAddress, bit_position: u32, n_bits: u32) -> u32 {
    assert!(n_bits > 0 && n_bits <= 32, "n_bits must be between 1 and 32");
    assert!(bit_position < 32, "bit_position must be less than 32");
    assert!(bit_position + n_bits <= 32, "bit range exceeds register size");

    unsafe {
        let reg_value = reg_read(reg_addr);
        let mask = ((1u64 << n_bits) - 1) as u32;
        (reg_value >> bit_position) & mask
    }
}

/// Clear a single bit (set it to 0).
///
/// Convenience wrapper for `reg_set_bit(reg_addr, bit_position, false)`.
pub fn reg_clr_bit(reg_addr: RegisterAddress, bit_position: u32) {
    reg_set_bit(reg_addr, bit_position, false);
}

/// Set a single bit (set it to 1).
///
/// Convenience wrapper for `reg_set_bit(reg_addr, bit_position, true)`.
pub fn reg_set_bit_high(reg_addr: RegisterAddress, bit_position: u32) {
    reg_set_bit(reg_addr, bit_position, true);
}

/// Clear specific bits using a mask placed at a bit position.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `clear_mask`: Bits set to 1 indicate which bits to clear
/// - `bit_position`: Where to place the mask (shift left by this amount)
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// // Clear bits 4, 5, and 6 (mask = 0b111)
/// reg_clr_val(0x4800_0000 as RegisterAddress, 0b111, 4);
/// ```
pub fn reg_clr_val(reg_addr: RegisterAddress, clear_mask: u32, bit_position: u32) {
    assert!(bit_position < 32, "bit_position must be less than 32");
    reg_assert_mask_fits(clear_mask, bit_position);
    
    unsafe {
        let reg_value = reg_read(reg_addr);
        let updated_value = reg_value & !((clear_mask) << bit_position);
        reg_write(reg_addr, updated_value);
    }
}

/// Write bits selected by `set_mask` at `bit_position`. Other bits stay the same.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `new_value`: New value for the masked bits (only bits in `set_mask` are used)
/// - `set_mask`: Which bits can change (1 = modifiable)
/// - `bit_position`: Where to place the field
///
/// Notes
/// - This performs a read‑modify‑write of the register. It is not atomic.
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// // Write 0b101 into bits 4..=6 (mask = 0b111)
/// reg_set_val_masked(0x4800_0000 as RegisterAddress, 0b101, 0b111, 4);
/// ```
pub fn reg_set_val_masked(reg_addr: RegisterAddress, new_value: u32, set_mask: u32, bit_position: u32) {
    assert!(bit_position < 32, "bit_position must be less than 32");
    reg_assert_mask_fits(set_mask, bit_position);

    // Ensure new_value only contains bits within set_mask.
    assert!(
        (new_value & !set_mask) == 0,
        "new_value has bits outside set_mask"
    );
    
    unsafe {
        // Single read‑modify‑write with proper masking
        let reg_value = reg_read(reg_addr);
        let mask = set_mask << bit_position;
        let updated_value = (reg_value & !mask) | ((new_value & set_mask) << bit_position);
        reg_write(reg_addr, updated_value);
    }
}

/// Read bits using a mask at a bit position.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `read_mask`: Mask to select bits after shifting right
/// - `bit_position`: How much to shift right before masking
///
/// Returns
/// - The masked and shifted value
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// // Read a 3‑bit value at bit 4 (mask = 0b111)
/// let value = reg_read_val_masked(0x4800_0000 as RegisterAddress, 0b111, 4);
/// ```
pub fn reg_read_val_masked(reg_addr: RegisterAddress, read_mask: u32, bit_position: u32) -> u32 {
    assert!(bit_position < 32, "bit_position must be less than 32");
    reg_assert_mask_fits(read_mask, bit_position);
    
    unsafe {
        let reg_value = reg_read(reg_addr);
        (reg_value >> bit_position) & read_mask
    }
}

/// Toggle (flip) one bit: 0 becomes 1, 1 becomes 0.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `bit_position`: Bit index (0..31)
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// reg_toggle_bit(0x4800_0000 as RegisterAddress, 5);
/// ```
pub fn reg_toggle_bit(reg_addr: RegisterAddress, bit_position: u32) {
    assert!(bit_position < 32, "bit_position must be less than 32");
    
    unsafe {
        let reg_value = reg_read(reg_addr);
        let updated_value = reg_value ^ (1u32 << bit_position);
        reg_write(reg_addr, updated_value);
    }
}

/// Toggle multiple bits using a mask placed at a bit position.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `toggle_mask`: Bits set to 1 will be toggled
/// - `bit_position`: Where to place the mask
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// // Toggle bits 4, 5, and 6 (mask = 0b111)
/// reg_toggle_bits(0x4800_0000 as RegisterAddress, 0b111, 4);
/// ```
pub fn reg_toggle_bits(reg_addr: RegisterAddress, toggle_mask: u32, bit_position: u32) {
    assert!(bit_position < 32, "bit_position must be less than 32");
    reg_assert_mask_fits(toggle_mask, bit_position);
    
    unsafe {
        let reg_value = reg_read(reg_addr);
        let updated_value = reg_value ^ (toggle_mask << bit_position);
        reg_write(reg_addr, updated_value);
    }
}

/// Read‑modify‑write using a user function.
///
/// How it works
/// - Reads the current value
/// - Calls `modify_fn(current)` to compute a new value
/// - Writes the new value back
///
/// Note
/// - This is not atomic. If interrupts or other code can also write this register,
///   use proper synchronization.
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// // Increment a 4‑bit field at bits 4..=7, wrapping at 15
/// reg_modify(0x4800_0000 as RegisterAddress, |val| {
///     let field = (val >> 4) & 0xF;
///     (val & !0xF0) | (((field + 1) & 0xF) << 4)
/// });
/// ```
pub fn reg_modify<F>(reg_addr: RegisterAddress, modify_fn: F) 
where 
    F: FnOnce(u32) -> u32,
{
    unsafe {
        let reg_value = reg_read(reg_addr);
        let new_value = modify_fn(reg_value);
        reg_write(reg_addr, new_value);
    }
}

/// Wait until one bit becomes a given value (busy‑wait/poll).
///
/// Arguments
/// - `reg_addr`: Register address
/// - `bit_position`: Bit index to check
/// - `expected_value`: true for 1, false for 0
/// - `timeout_cycles`: Max read attempts before giving up (0 = wait forever)
///
/// Returns
/// - true if the bit matched in time, false if it timed out
///
/// Notes
/// - Uses `core::hint::spin_loop()` to hint the CPU in the wait loop.
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// let ok = reg_wait_bit(0x4800_0000 as RegisterAddress, 0, true, 1000);
/// ```
#[must_use]
pub fn reg_wait_bit(reg_addr: RegisterAddress, bit_position: u32, expected_value: bool, timeout_cycles: u32) -> bool {
    assert!(bit_position < 32, "bit_position must be less than 32");
    
    let mut cycles = 0;
    
    loop {
        if reg_read_bit(reg_addr, bit_position) == expected_value {
            return true;
        }
        hint::spin_loop();
        if timeout_cycles > 0 {
            cycles += 1;
            if cycles >= timeout_cycles {
                return false;
            }
        }
    }
}

/// Wait until selected bits match an expected value (busy‑wait/poll).
///
/// Arguments
/// - `reg_addr`: Register address
/// - `expected_value`: Expected value after masking and shifting
/// - `mask`: Which bits to check (before shifting)
/// - `bit_position`: Where the field starts
/// - `timeout_cycles`: Max read attempts before giving up (0 = wait forever)
///
/// Returns
/// - true if the bits matched in time, false if it timed out
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// let ok = reg_wait_bits(0x4800_0000 as RegisterAddress, 0b101, 0b111, 4, 500);
/// ```
#[must_use]
pub fn reg_wait_bits(reg_addr: RegisterAddress, expected_value: u32, mask: u32, bit_position: u32, timeout_cycles: u32) -> bool {
    assert!(bit_position < 32, "bit_position must be less than 32");
    
    let mut cycles = 0;
    
    loop {
        let current_value = reg_read_val_masked(reg_addr, mask, bit_position);
        if current_value == expected_value {
            return true;
        }
        hint::spin_loop();
        if timeout_cycles > 0 {
            cycles += 1;
            if cycles >= timeout_cycles {
                return false;
            }
        }
    }
}

/// Test‑and‑set one bit: return the old value, then set it to 1.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `bit_position`: Bit index (0..31)
///
/// Returns
/// - The previous value of the bit
///
/// Note
/// - Not atomic. Use synchronization if required.
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// let was_set = reg_test_and_set(0x4800_0000 as RegisterAddress, 7);
/// ```
#[must_use]
pub fn reg_test_and_set(reg_addr: RegisterAddress, bit_position: u32) -> bool {
    assert!(bit_position < 32, "bit_position must be less than 32");
    
    unsafe {
        let reg_value = reg_read(reg_addr);
        let bit_was_set = (reg_value & (1u32 << bit_position)) != 0;
        let updated_value = reg_value | (1u32 << bit_position);
        reg_write(reg_addr, updated_value);
        bit_was_set
    }
}

/// Test‑and‑clear one bit: return the old value, then clear it to 0.
///
/// Arguments
/// - `reg_addr`: Register address
/// - `bit_position`: Bit index (0..31)
///
/// Returns
/// - The previous value of the bit
///
/// Note
/// - Not atomic. Use synchronization if required.
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// let was_set = reg_test_and_clear(0x4800_0000 as RegisterAddress, 3);
/// ```
#[must_use]
pub fn reg_test_and_clear(reg_addr: RegisterAddress, bit_position: u32) -> bool {
    assert!(bit_position < 32, "bit_position must be less than 32");
    
    unsafe {
        let reg_value = reg_read(reg_addr);
        let bit_was_set = (reg_value & (1u32 << bit_position)) != 0;
        let updated_value = reg_value & !(1u32 << bit_position);
        reg_write(reg_addr, updated_value);
        bit_was_set
    }
}

/// Count how many bits are set to 1 in the register.
///
/// Arguments
/// - `reg_addr`: Register address
///
/// Returns
/// - Number of 1 bits
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// let count = reg_count_set_bits(0x4800_0000 as RegisterAddress);
/// ```
pub fn reg_count_set_bits(reg_addr: RegisterAddress) -> u32 {
    unsafe {
        let reg_value = reg_read(reg_addr);
        reg_value.count_ones()
    }
}

/// Find the position of the first set bit (least significant 1).
///
/// Arguments
/// - `reg_addr`: Register address
///
/// Returns
/// - Some(position) if a 1 bit exists, None if the value is 0
///
/// Safety
/// - Only use valid hardware register addresses.
///
/// Example
/// ```ignore
/// if let Some(pos) = reg_find_first_set(0x4800_0000 as RegisterAddress) {
///     // use pos
/// }
/// ```
pub fn reg_find_first_set(reg_addr: RegisterAddress) -> Option<u32> {
    unsafe {
        let reg_value = reg_read(reg_addr);
        if reg_value == 0 {
            None
        } else {
            Some(reg_value.trailing_zeros())
        }
    }
}

/// Legacy and in-place helpers
///
/// Purpose
/// - Compatibility with older C-style macros and for unit tests.
/// - Operate on a u32 value in RAM via volatile reads/writes (no MMIO pointer).
///
/// Prefer instead
/// - The address-based APIs above for real peripherals:
///   reg_read, reg_write, reg_set_bit, reg_set_bits, reg_set_val_masked, etc.
///
/// Use when
/// - Working with a shadow copy in RAM or writing small tests/benchmarks.
///
/// Notes
/// - reg_write/reg_read are deprecated; use reg_set_val/reg_read instead.
/// - These helpers do not access hardware by themselves; they only modify the passed value.
#[deprecated(note = "Use reg_set_val instead")]
#[inline(always)]
pub fn reg_write_inplace(reg: &mut u32, val: u32) {
    unsafe { ptr::write_volatile(reg, val) }
}

#[deprecated(note = "Use reg_read instead")]
#[inline(always)]
pub fn reg_read_inplace(reg: &u32) -> u32 {
    unsafe { ptr::read_volatile(reg) }
}

#[inline(always)]
pub fn reg_set_bit_inplace(reg: &mut u32, pos: u32) {
    assert!(pos < 32, "bit_position must be less than 32");
    unsafe { core::ptr::write_volatile(reg, core::ptr::read_volatile(reg) | (1u32 << pos)) }
}

#[inline(always)]
pub fn reg_clr_bit_inplace(reg: &mut u32, pos: u32) {
    assert!(pos < 32, "bit_position must be less than 32");
    unsafe { core::ptr::write_volatile(reg, core::ptr::read_volatile(reg) & !(1u32 << pos)) }
}

#[inline(always)]
pub fn reg_toggle_bit_inplace(reg: &mut u32, pos: u32) {
    assert!(pos < 32, "bit_position must be less than 32");
    unsafe { core::ptr::write_volatile(reg, core::ptr::read_volatile(reg) ^ (1u32 << pos)) }
}

#[inline(always)]
pub fn reg_read_bit_inplace(reg: &u32, pos: u32) -> bool {
    assert!(pos < 32, "bit_position must be less than 32");
    (unsafe { core::ptr::read_volatile(reg) } & (1u32 << pos)) != 0
}

#[inline(always)]
pub fn reg_clr_val_inplace(reg: &mut u32, clrmask: u32, pos: u32) {
    assert!(pos < 32, "bit_position must be less than 32");
    reg_assert_mask_fits(clrmask, pos);
    unsafe {
        core::ptr::write_volatile(reg, core::ptr::read_volatile(reg) & !(clrmask << pos));
    }
}

#[inline(always)]
pub fn reg_set_val_inplace(reg: &mut u32, val: u32, setmask: u32, pos: u32) {
    assert!(pos < 32, "bit_position must be less than 32");
    reg_assert_mask_fits(setmask, pos);
    // Safer than the C macro: enforce val fits in setmask.
    assert!((val & !setmask) == 0, "val has bits outside setmask");
    unsafe {
        let current = core::ptr::read_volatile(reg);
        let mask = setmask << pos;
        let updated = (current & !mask) | ((val & setmask) << pos);
        core::ptr::write_volatile(reg, updated);
    }
}

#[inline(always)]
pub fn reg_read_val_inplace(reg: &u32, rdmask: u32, pos: u32) -> u32 {
    assert!(pos < 32, "bit_position must be less than 32");
    reg_assert_mask_fits(rdmask, pos);
    (unsafe { core::ptr::read_volatile(reg) } >> pos) & rdmask
}
