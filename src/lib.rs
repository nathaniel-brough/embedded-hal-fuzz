//! This crate is specifically designed for fuzzing device drivers or full apps.
//! It provides a best guess for how to fuzz device-drivers/apps.
//!
//! # Getting started
//! If you are not familiar with fuzzing in rust then it is recommended that you
//! read over the [cargo-fuzz book](https://rust-fuzz.github.io/book/cargo-fuzz.html).
//!
//! ## Install cargo-fuzz
//! To install cargo-fuzz, run:
//! ```bash
//! cargo install cargo-fuzz
//! ```
//!
//! ## Initialising cargo fuzz in your project
//! To set your project up to use cargo fuzz, run:
//! ```bash
//! cargo-fuzz init
//! ```
//! This will add a set to targets under the 'fuzz' directory.
//!
//! ## Adding a new embedded fuzz target
//! To add a new embedded fuzz target, run:
//! ```bash
//! cargo-fuzz add <my_target>
//! ```
//! This will add a new binary target 'fuzz/fuzz_targets/my_target.rs'.
//! by default this your new target will look something like this;
//! ```no_run
//! #![no_main]
//! use libfuzzer_sys::fuzz_target;
//!
//! fuzz_target!(|data: &[u8]| {
//!     // fuzzed code goes here
//! });
//! ```
//!
//! To use this library simply bundle all the types that you need into
//! a fuzzing context object e.g.
//! ```rust,no_run
//! use libfuzzer_sys::fuzz_target;
//! use embedded_hal_fuzz::digital::{ArbitraryInputPin, ArbitraryOutputPin};
//! use embedded_hal_fuzz::spi::ArbitrarySpiBus;
//! use embedded_hal::spi::SpiBus;
//! use embedded_hal::digital::{InputPin, OutputPin};
//! use arbitrary::Arbitrary;
//!
//! #[derive(Debug, Arbitrary)]
//! struct Ctx {
//!   input_pin: ArbitraryInputPin,
//!   output_pin: ArbitraryOutputPin,
//!   spi: ArbitrarySpiBus<u16>,
//!   other_data: Vec<u8>,
//! }
//! fuzz_target!(|ctx: Ctx| {
//!   let Ctx {input_pin, mut output_pin, mut spi, other_data } = ctx;
//!   let _ = output_pin.set_high();
//! });
//! ```
//!
//! Each of these fuzzed peripherals will return arbitrary results including
//! both Ok/Error types. As these inputs are driven by fuzzed data,
//! these types are perfect for fuzzing your drivers.

pub mod delay;
pub mod digital;
pub mod i2c;
pub mod pwm;
pub mod spi;
