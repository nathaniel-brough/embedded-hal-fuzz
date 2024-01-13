//! To make use of this module you can simply pass
//! this in as a value from the fuzz_target macro e.g.
//! ```rust
//! use libfuzzer_sys::fuzz_target;
//! use embedded_hal_fuzz::delay::ArbitraryDelayNs;
//! use embedded_hal::delay::DelayNs;
//! fuzz_target!(|delay_handle: ArbitraryDelayNs| {
//!   let mut delay_handle = delay_handle;
//!   delay_handle.delay_us(10);
//! });
//! ```
use arbitrary::Arbitrary;
use embedded_hal::delay::DelayNs;

/// An arbitrary delay implmentation that is a no-op
/// when we are fuzzing we typically don't want to
/// actually delay anything as that would slow things
/// down. We also shouldn't support API's that use
/// delays to avoid race-conditions as this is poor
/// design.
#[derive(Debug, Arbitrary)]
pub struct ArbitraryDelayNs;

impl DelayNs for ArbitraryDelayNs {
    fn delay_ns(&mut self, _us: u32) {
        // no-op
    }
    fn delay_ms(&mut self, _ms: u32) {
        // no-op
    }
}
