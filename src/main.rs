/*
 * @file main.rs
 * @brief DAY002 - Blink Multiple LEDs in Sequence
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

//! FILE: main.rs
//!
//! DESCRIPTION:
//! DAY002 - RP2350 Embedded Rust Embassy Multiple LED Sequence Application.
//!
//! BRIEF:
//! Main application entry point for RP2350 GPIO LED sequence driver using Embassy.
//! Implements async sequential LED blinking on GPIO pins 16, 17, 18, 19.
//! Part of the 365 Pico2 RP2350 Project Ideas series.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 7, 2025
//! UPDATE DATE: December 7, 2025

#![no_std]
#![no_main]

mod config;
mod led;

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use led::{led_state_to_level, LedSequenceController};
use panic_halt as _;

/// Main application entry point.
///
/// # Details
/// Initializes Embassy runtime and runs the main LED sequence loop.
/// Uses LedSequenceController for state management.
/// Controls 4 LEDs on GPIO pins 16, 17, 18, 19 in sequence.
///
/// # Arguments
/// * `_spawner` - Embassy task spawner (reserved for future async tasks).
///
/// # Returns
/// * `()` - Never returns (infinite loop).
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led0 = Output::new(p.PIN_16, Level::Low);
    let mut led1 = Output::new(p.PIN_17, Level::Low);
    let mut led2 = Output::new(p.PIN_18, Level::Low);
    let mut led3 = Output::new(p.PIN_19, Level::Low);
    let mut controller = LedSequenceController::new();
    loop {
        if led_state_to_level(controller.led_state(0)) {
            led0.set_high();
        } else {
            led0.set_low();
        }
        if led_state_to_level(controller.led_state(1)) {
            led1.set_high();
        } else {
            led1.set_low();
        }
        if led_state_to_level(controller.led_state(2)) {
            led2.set_high();
        } else {
            led2.set_low();
        }
        if led_state_to_level(controller.led_state(3)) {
            led3.set_high();
        } else {
            led3.set_low();
        }
        Timer::after_millis(controller.delay_ms()).await;
        controller.advance();
    }
}
