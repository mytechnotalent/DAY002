/*
 * @file led.rs
 * @brief LED sequence state management
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

//! FILE: led.rs
//!
//! DESCRIPTION:
//! LED Sequence State Management for RP2350.
//!
//! BRIEF:
//! Provides LED sequence controller for blinking multiple LEDs.
//! Manages sequential LED activation with configurable timing.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 7, 2025
//! UPDATE DATE: December 7, 2025

use crate::config::{LED_COUNT, SEQUENCE_DELAY_MS};

/// LED state enumeration.
///
/// # Details
/// Represents the current state of an individual LED.
/// Used for state tracking and transitions.
///
/// # Variants
/// * `On` - LED is currently on (high)
/// * `Off` - LED is currently off (low)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LedState {
    On,
    Off,
}

/// LED sequence controller with state tracking.
///
/// # Details
/// Maintains LED sequence state and timing configuration.
/// Provides methods for advancing through LED sequence.
///
/// # Fields
/// * `current_index` - Index of currently active LED (0 to LED_COUNT-1)
/// * `led_count` - Total number of LEDs in sequence
/// * `delay_ms` - Delay between LED transitions in milliseconds
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct LedSequenceController {
    current_index: usize,
    led_count: usize,
    delay_ms: u64,
}

impl Default for LedSequenceController {
    /// Returns default LedSequenceController instance.
    ///
    /// # Details
    /// Delegates to new() for initialization.
    ///
    /// # Returns
    /// * `Self` - New LedSequenceController with default values
    #[allow(dead_code)]
    fn default() -> Self {
        Self::new()
    }
}

impl LedSequenceController {
    /// Creates new LED sequence controller with default settings.
    ///
    /// # Details
    /// Initializes controller starting at first LED (index 0).
    ///
    /// # Returns
    /// * `Self` - New LedSequenceController instance
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            current_index: 0,
            led_count: LED_COUNT,
            delay_ms: SEQUENCE_DELAY_MS,
        }
    }

    /// Advances to next LED in sequence and returns new index.
    ///
    /// # Details
    /// Moves to next LED, wrapping around to first LED after last.
    /// Implements circular sequence behavior.
    ///
    /// # Returns
    /// * `usize` - New LED index after advancement
    #[allow(dead_code)]
    pub fn advance(&mut self) -> usize {
        self.current_index = (self.current_index + 1) % self.led_count;
        self.current_index
    }

    /// Returns current LED index.
    ///
    /// # Details
    /// Index of LED that should currently be on.
    ///
    /// # Returns
    /// * `usize` - Current LED index (0 to LED_COUNT-1)
    #[allow(dead_code)]
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Returns total LED count.
    ///
    /// # Details
    /// Number of LEDs in the sequence.
    ///
    /// # Returns
    /// * `usize` - Total LED count
    #[allow(dead_code)]
    pub fn led_count(&self) -> usize {
        self.led_count
    }

    /// Returns current sequence delay.
    ///
    /// # Details
    /// Delay used for sequence timing in milliseconds.
    ///
    /// # Returns
    /// * `u64` - Delay in milliseconds
    #[allow(dead_code)]
    pub fn delay_ms(&self) -> u64 {
        self.delay_ms
    }

    /// Returns LED state for given index.
    ///
    /// # Details
    /// Returns On if index matches current, Off otherwise.
    ///
    /// # Arguments
    /// * `index` - LED index to check
    ///
    /// # Returns
    /// * `LedState` - On if current, Off otherwise
    #[allow(dead_code)]
    pub fn led_state(&self, index: usize) -> LedState {
        if index == self.current_index {
            LedState::On
        } else {
            LedState::Off
        }
    }
}

/// Converts LedState to boolean for GPIO control.
///
/// # Details
/// Maps On state to true (high), Off state to false (low).
///
/// # Arguments
/// * `state` - LED state to convert
///
/// # Returns
/// * `bool` - true for On, false for Off
#[allow(dead_code)]
pub fn led_state_to_level(state: LedState) -> bool {
    matches!(state, LedState::On)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== LedState Enum Tests ====================

    #[test]
    fn test_led_state_on_exists() {
        let _state = LedState::On;
    }

    #[test]
    fn test_led_state_off_exists() {
        let _state = LedState::Off;
    }

    #[test]
    fn test_led_state_equality_on() {
        assert_eq!(LedState::On, LedState::On);
    }

    #[test]
    fn test_led_state_equality_off() {
        assert_eq!(LedState::Off, LedState::Off);
    }

    #[test]
    fn test_led_state_inequality() {
        assert_ne!(LedState::On, LedState::Off);
        assert_ne!(LedState::Off, LedState::On);
    }

    #[test]
    fn test_led_state_copy() {
        let state = LedState::On;
        let copy = state;
        assert_eq!(state, copy);
    }

    #[test]
    fn test_led_state_clone() {
        let state = LedState::Off;
        #[allow(clippy::clone_on_copy)]
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_led_state_debug_on() {
        let state = LedState::On;
        let debug_str = format!("{:?}", state);
        assert_eq!(debug_str, "On");
    }

    #[test]
    fn test_led_state_debug_off() {
        let state = LedState::Off;
        let debug_str = format!("{:?}", state);
        assert_eq!(debug_str, "Off");
    }

    #[test]
    fn test_led_state_size() {
        assert_eq!(core::mem::size_of::<LedState>(), 1);
    }

    // ==================== led_state_to_level Function Tests ====================

    #[test]
    fn test_led_state_to_level_on() {
        assert!(led_state_to_level(LedState::On));
    }

    #[test]
    fn test_led_state_to_level_off() {
        assert!(!led_state_to_level(LedState::Off));
    }

    #[test]
    fn test_led_state_to_level_on_returns_true() {
        assert_eq!(led_state_to_level(LedState::On), true);
    }

    #[test]
    fn test_led_state_to_level_off_returns_false() {
        assert_eq!(led_state_to_level(LedState::Off), false);
    }

    #[test]
    fn test_led_state_to_level_consistent() {
        for _ in 0..10 {
            assert!(led_state_to_level(LedState::On));
            assert!(!led_state_to_level(LedState::Off));
        }
    }

    // ==================== LedSequenceController::new() Tests ====================

    #[test]
    fn test_new_controller() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl.delay_ms(), SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_new_controller_starts_at_zero() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl.current_index(), 0);
    }

    #[test]
    fn test_new_controller_led_count() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl.led_count(), LED_COUNT);
    }

    #[test]
    fn test_new_controller_state() {
        let ctrl = LedSequenceController::new();
        let expected = LedSequenceController {
            current_index: 0,
            led_count: LED_COUNT,
            delay_ms: SEQUENCE_DELAY_MS,
        };
        assert_eq!(ctrl, expected);
    }

    #[test]
    fn test_new_returns_consistent_value() {
        let ctrl1 = LedSequenceController::new();
        let ctrl2 = LedSequenceController::new();
        assert_eq!(ctrl1, ctrl2);
    }

    // ==================== LedSequenceController::default() Tests ====================

    #[test]
    fn test_default_equals_new() {
        let default = LedSequenceController::default();
        let new = LedSequenceController::new();
        assert_eq!(default, new);
    }

    #[test]
    fn test_default_delay() {
        let default = LedSequenceController::default();
        assert_eq!(default.delay_ms(), SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_default_starts_at_zero() {
        let default = LedSequenceController::default();
        assert_eq!(default.current_index(), 0);
    }

    // ==================== LedSequenceController::advance() Tests ====================

    #[test]
    fn test_advance_from_zero() {
        let mut ctrl = LedSequenceController::new();
        assert_eq!(ctrl.advance(), 1);
    }

    #[test]
    fn test_advance_increments() {
        let mut ctrl = LedSequenceController::new();
        ctrl.advance();
        assert_eq!(ctrl.current_index(), 1);
    }

    #[test]
    fn test_advance_returns_new_index() {
        let mut ctrl = LedSequenceController::new();
        let new_index = ctrl.advance();
        assert_eq!(new_index, ctrl.current_index());
    }

    #[test]
    fn test_advance_wraps_around() {
        let mut ctrl = LedSequenceController::new();
        for _ in 0..LED_COUNT {
            ctrl.advance();
        }
        assert_eq!(ctrl.current_index(), 0);
    }

    #[test]
    fn test_advance_full_cycle() {
        let mut ctrl = LedSequenceController::new();
        for i in 0..LED_COUNT {
            assert_eq!(ctrl.current_index(), i);
            ctrl.advance();
        }
        assert_eq!(ctrl.current_index(), 0);
    }

    #[test]
    fn test_advance_multiple_cycles() {
        let mut ctrl = LedSequenceController::new();
        for cycle in 0..3 {
            for i in 0..LED_COUNT {
                let expected = i;
                assert_eq!(
                    ctrl.current_index(),
                    expected,
                    "Cycle {} index {}",
                    cycle,
                    i
                );
                ctrl.advance();
            }
        }
    }

    #[test]
    fn test_advance_preserves_delay() {
        let mut ctrl = LedSequenceController::new();
        let delay_before = ctrl.delay_ms();
        ctrl.advance();
        ctrl.advance();
        ctrl.advance();
        assert_eq!(ctrl.delay_ms(), delay_before);
    }

    // ==================== LedSequenceController::current_index() Tests ====================

    #[test]
    fn test_current_index_initial() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl.current_index(), 0);
    }

    #[test]
    fn test_current_index_after_advance() {
        let mut ctrl = LedSequenceController::new();
        ctrl.advance();
        assert_eq!(ctrl.current_index(), 1);
    }

    #[test]
    fn test_current_index_within_bounds() {
        let mut ctrl = LedSequenceController::new();
        for _ in 0..100 {
            assert!(ctrl.current_index() < LED_COUNT);
            ctrl.advance();
        }
    }

    // ==================== LedSequenceController::led_count() Tests ====================

    #[test]
    fn test_led_count_returns_correct_value() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl.led_count(), LED_COUNT);
    }

    #[test]
    fn test_led_count_is_4() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl.led_count(), 4);
    }

    #[test]
    fn test_led_count_immutable_after_advance() {
        let mut ctrl = LedSequenceController::new();
        ctrl.advance();
        assert_eq!(ctrl.led_count(), LED_COUNT);
    }

    // ==================== LedSequenceController::delay_ms() Tests ====================

    #[test]
    fn test_delay_ms_returns_correct_value() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl.delay_ms(), SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_delay_ms_is_250() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl.delay_ms(), 250);
    }

    #[test]
    fn test_delay_ms_immutable_after_advance() {
        let mut ctrl = LedSequenceController::new();
        ctrl.advance();
        assert_eq!(ctrl.delay_ms(), SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_delay_ms_within_config_range() {
        let ctrl = LedSequenceController::new();
        assert!(ctrl.delay_ms() >= crate::config::MIN_SEQUENCE_DELAY_MS);
        assert!(ctrl.delay_ms() <= crate::config::MAX_SEQUENCE_DELAY_MS);
    }

    // ==================== LedSequenceController::led_state() Tests ====================

    #[test]
    fn test_led_state_current_is_on() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl.led_state(0), LedState::On);
    }

    #[test]
    fn test_led_state_others_are_off() {
        let ctrl = LedSequenceController::new();
        for i in 1..LED_COUNT {
            assert_eq!(ctrl.led_state(i), LedState::Off);
        }
    }

    #[test]
    fn test_led_state_after_advance() {
        let mut ctrl = LedSequenceController::new();
        ctrl.advance();
        assert_eq!(ctrl.led_state(0), LedState::Off);
        assert_eq!(ctrl.led_state(1), LedState::On);
    }

    #[test]
    fn test_led_state_only_one_on() {
        let ctrl = LedSequenceController::new();
        let on_count: usize = (0..LED_COUNT)
            .filter(|&i| ctrl.led_state(i) == LedState::On)
            .count();
        assert_eq!(on_count, 1);
    }

    #[test]
    fn test_led_state_all_others_off() {
        let ctrl = LedSequenceController::new();
        let off_count: usize = (0..LED_COUNT)
            .filter(|&i| ctrl.led_state(i) == LedState::Off)
            .count();
        assert_eq!(off_count, LED_COUNT - 1);
    }

    // ==================== LedSequenceController Struct Tests ====================

    #[test]
    fn test_initial_state() {
        let ctrl = LedSequenceController::new();
        let expected = LedSequenceController {
            current_index: 0,
            led_count: LED_COUNT,
            delay_ms: SEQUENCE_DELAY_MS,
        };
        assert_eq!(ctrl, expected);
    }

    #[test]
    fn test_controller_size() {
        assert!(core::mem::size_of::<LedSequenceController>() <= 32);
    }

    #[test]
    fn test_controller_alignment() {
        assert!(core::mem::align_of::<LedSequenceController>() <= 8);
    }

    // ==================== Trait Implementation Tests ====================

    #[test]
    fn test_led_state_debug() {
        let state = LedState::On;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("On"));
    }

    #[test]
    fn test_controller_clone() {
        let ctrl1 = LedSequenceController::new();
        let ctrl2 = ctrl1;
        assert_eq!(ctrl1.delay_ms(), ctrl2.delay_ms());
    }

    #[test]
    fn test_controller_copy() {
        let ctrl1 = LedSequenceController::new();
        let ctrl2 = ctrl1;
        assert_eq!(ctrl1, ctrl2);
    }

    #[test]
    fn test_controller_partial_eq() {
        let ctrl1 = LedSequenceController::new();
        let ctrl2 = LedSequenceController::new();
        assert_eq!(ctrl1, ctrl2);
    }

    #[test]
    fn test_controller_eq_reflexive() {
        let ctrl = LedSequenceController::new();
        assert_eq!(ctrl, ctrl);
    }

    #[test]
    fn test_controller_eq_symmetric() {
        let ctrl1 = LedSequenceController::new();
        let ctrl2 = LedSequenceController::new();
        assert_eq!(ctrl1, ctrl2);
        assert_eq!(ctrl2, ctrl1);
    }

    #[test]
    fn test_controller_debug() {
        let ctrl = LedSequenceController::new();
        let debug_str = format!("{:?}", ctrl);
        assert!(debug_str.contains("LedSequenceController"));
    }

    #[test]
    fn test_controller_debug_contains_index() {
        let ctrl = LedSequenceController::new();
        let debug_str = format!("{:?}", ctrl);
        assert!(debug_str.contains("0"));
    }

    #[test]
    fn test_controller_debug_contains_delay() {
        let ctrl = LedSequenceController::new();
        let debug_str = format!("{:?}", ctrl);
        assert!(debug_str.contains("250"));
    }

    // ==================== State Transition Tests ====================

    #[test]
    fn test_sequence_cycle() {
        let mut ctrl = LedSequenceController::new();
        assert_eq!(ctrl.current_index(), 0);
        ctrl.advance();
        assert_eq!(ctrl.current_index(), 1);
        ctrl.advance();
        assert_eq!(ctrl.current_index(), 2);
        ctrl.advance();
        assert_eq!(ctrl.current_index(), 3);
        ctrl.advance();
        assert_eq!(ctrl.current_index(), 0);
    }

    #[test]
    fn test_independent_controllers() {
        let mut ctrl1 = LedSequenceController::new();
        let ctrl2 = LedSequenceController::new();
        ctrl1.advance();
        ctrl1.advance();
        assert_eq!(ctrl2.current_index(), 0);
        assert_eq!(ctrl1.current_index(), 2);
    }

    #[test]
    fn test_advance_and_level_consistency() {
        let mut ctrl = LedSequenceController::new();
        for _ in 0..LED_COUNT * 3 {
            let current = ctrl.current_index();
            assert_eq!(led_state_to_level(ctrl.led_state(current)), true);
            for i in 0..LED_COUNT {
                if i != current {
                    assert_eq!(led_state_to_level(ctrl.led_state(i)), false);
                }
            }
            ctrl.advance();
        }
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_many_controllers() {
        let controllers: Vec<LedSequenceController> =
            (0..100).map(|_| LedSequenceController::new()).collect();
        for ctrl in controllers {
            assert_eq!(ctrl.delay_ms(), SEQUENCE_DELAY_MS);
        }
    }

    #[test]
    fn test_controller_in_option() {
        let maybe_ctrl: Option<LedSequenceController> = Some(LedSequenceController::new());
        assert!(maybe_ctrl.is_some());
        assert_eq!(maybe_ctrl.unwrap().delay_ms(), SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_controller_in_result() {
        let result: Result<LedSequenceController, ()> = Ok(LedSequenceController::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().delay_ms(), SEQUENCE_DELAY_MS);
    }

    #[test]
    fn test_advance_100_times() {
        let mut ctrl = LedSequenceController::new();
        for i in 0..100 {
            let expected_index = i % LED_COUNT;
            assert_eq!(ctrl.current_index(), expected_index);
            ctrl.advance();
        }
    }
}
