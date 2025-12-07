/*
 * @file config.rs
 * @brief Application configuration constants
 * @author Kevin Thomas
 * @date 2025
 *
 * MIT License
 *
 * Copyright (c) 2025 Kevin Thomas
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! FILE: config.rs
//!
//! DESCRIPTION:
//! RP2350 Multiple LED Sequence Configuration Constants.
//!
//! BRIEF:
//! Defines configuration constants for LED sequence timing.
//! Contains delay intervals, LED count, and GPIO pin configuration.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 7, 2025
//! UPDATE DATE: December 7, 2025

/// Number of LEDs in the sequence.
///
/// # Details
/// Defines the total number of LEDs to blink in sequence.
/// Uses 4 LEDs connected to consecutive GPIO pins.
///
/// # Value
/// 4 LEDs
#[allow(dead_code)]
pub const LED_COUNT: usize = 4;

/// Default LED sequence delay in milliseconds.
///
/// # Details
/// Configures the delay between LED state transitions in sequence.
/// Each LED stays on for this duration before the next lights up.
///
/// # Value
/// 250 milliseconds
#[allow(dead_code)]
pub const SEQUENCE_DELAY_MS: u64 = 250;

/// Minimum allowed sequence delay in milliseconds.
///
/// # Details
/// Prevents excessively fast sequencing which may cause issues.
///
/// # Value
/// 10 milliseconds
#[allow(dead_code)]
pub const MIN_SEQUENCE_DELAY_MS: u64 = 10;

/// Maximum allowed sequence delay in milliseconds.
///
/// # Details
/// Prevents excessively slow sequencing for practical use.
///
/// # Value
/// 5000 milliseconds (5 seconds)
#[allow(dead_code)]
pub const MAX_SEQUENCE_DELAY_MS: u64 = 5000;

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== LED_COUNT Tests ====================

    #[test]
    fn test_led_count_value() {
        assert_eq!(LED_COUNT, 4);
    }

    #[test]
    fn test_led_count_is_usize() {
        let _: usize = LED_COUNT;
    }

    #[test]
    fn test_led_count_non_zero() {
        assert!(LED_COUNT > 0);
    }

    #[test]
    fn test_led_count_reasonable() {
        assert!(LED_COUNT >= 2);
        assert!(LED_COUNT <= 24);
    }

    // ==================== SEQUENCE_DELAY_MS Tests ====================

    #[test]
    fn test_sequence_delay_default() {
        assert_eq!(SEQUENCE_DELAY_MS, 250);
    }

    #[test]
    fn test_sequence_delay_is_u64() {
        let _: u64 = SEQUENCE_DELAY_MS;
    }

    #[test]
    fn test_sequence_delay_non_zero() {
        assert!(SEQUENCE_DELAY_MS > 0);
    }

    #[test]
    fn test_sequence_delay_reasonable_range() {
        assert!(SEQUENCE_DELAY_MS >= 50);
        assert!(SEQUENCE_DELAY_MS <= 2000);
    }

    // ==================== MIN_SEQUENCE_DELAY_MS Tests ====================

    #[test]
    fn test_min_delay_value() {
        assert_eq!(MIN_SEQUENCE_DELAY_MS, 10);
    }

    #[test]
    fn test_min_delay_is_u64() {
        let _: u64 = MIN_SEQUENCE_DELAY_MS;
    }

    #[test]
    fn test_min_delay_non_zero() {
        assert!(MIN_SEQUENCE_DELAY_MS > 0);
    }

    #[test]
    fn test_min_delay_less_than_default() {
        assert!(MIN_SEQUENCE_DELAY_MS < SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_min_delay_practical_minimum() {
        assert!(MIN_SEQUENCE_DELAY_MS >= 1);
    }

    // ==================== MAX_SEQUENCE_DELAY_MS Tests ====================

    #[test]
    fn test_max_delay_value() {
        assert_eq!(MAX_SEQUENCE_DELAY_MS, 5000);
    }

    #[test]
    fn test_max_delay_is_u64() {
        let _: u64 = MAX_SEQUENCE_DELAY_MS;
    }

    #[test]
    fn test_max_delay_greater_than_default() {
        assert!(MAX_SEQUENCE_DELAY_MS > SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_max_delay_is_5_seconds() {
        assert_eq!(MAX_SEQUENCE_DELAY_MS, 5 * 1000);
    }

    // ==================== Range Relationship Tests ====================

    #[test]
    fn test_delay_range_valid() {
        assert!(MIN_SEQUENCE_DELAY_MS < MAX_SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_default_within_range() {
        assert!(SEQUENCE_DELAY_MS >= MIN_SEQUENCE_DELAY_MS);
        assert!(SEQUENCE_DELAY_MS <= MAX_SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_range_span() {
        let span = MAX_SEQUENCE_DELAY_MS - MIN_SEQUENCE_DELAY_MS;
        assert!(span > 0);
        assert_eq!(span, 4990);
    }

    #[test]
    fn test_default_not_at_boundaries() {
        assert_ne!(SEQUENCE_DELAY_MS, MIN_SEQUENCE_DELAY_MS);
        assert_ne!(SEQUENCE_DELAY_MS, MAX_SEQUENCE_DELAY_MS);
    }

    // ==================== Arithmetic Safety Tests ====================

    #[test]
    fn test_no_overflow_on_double() {
        let doubled = SEQUENCE_DELAY_MS.checked_mul(2);
        assert!(doubled.is_some());
    }

    #[test]
    fn test_no_overflow_max_doubled() {
        let doubled = MAX_SEQUENCE_DELAY_MS.checked_mul(2);
        assert!(doubled.is_some());
    }

    #[test]
    fn test_min_subtraction_safe() {
        let result = SEQUENCE_DELAY_MS.checked_sub(MIN_SEQUENCE_DELAY_MS);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 240);
    }

    #[test]
    fn test_values_fit_in_u32() {
        assert!(SEQUENCE_DELAY_MS <= u32::MAX as u64);
        assert!(MIN_SEQUENCE_DELAY_MS <= u32::MAX as u64);
        assert!(MAX_SEQUENCE_DELAY_MS <= u32::MAX as u64);
    }

    // ==================== Constant Immutability Tests ====================

    #[test]
    fn test_constants_are_const() {
        const _A: usize = LED_COUNT;
        const _B: u64 = SEQUENCE_DELAY_MS;
        const _C: u64 = MIN_SEQUENCE_DELAY_MS;
        const _D: u64 = MAX_SEQUENCE_DELAY_MS;
    }

    #[test]
    fn test_constants_usable_in_const_context() {
        const DOUBLE_DELAY: u64 = SEQUENCE_DELAY_MS * 2;
        assert_eq!(DOUBLE_DELAY, 500);
    }

    #[test]
    fn test_led_count_usable_in_array() {
        let _arr: [u8; LED_COUNT] = [0; LED_COUNT];
    }

    // ==================== LED Count Relationship Tests ====================

    #[test]
    fn test_total_sequence_time() {
        let total = SEQUENCE_DELAY_MS * LED_COUNT as u64;
        assert_eq!(total, 1000);
    }

    #[test]
    fn test_led_count_fits_in_u8() {
        assert!(LED_COUNT <= u8::MAX as usize);
    }
}
