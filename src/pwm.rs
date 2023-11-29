//! To make use of this module you can simply pass
//! this in as a value from the fuzz_target macro e.g.
//!
//! # Example
//!
//! ```rust
//! use libfuzzer_sys::fuzz_target;
//! use embedded_hal_fuzz::pwm::ArbitraryPwm;
//! use embedded_hal::pwm::SetDutyCycle;
//!
//! fuzz_target!(|pwm: ArbitraryPwm| {
//!   let mut pwm = pwm;
//!   let _ = pwm.set_duty_cycle(10);
//! });
//! ```

use arbitrary::Arbitrary;
use embedded_hal::pwm::{self, ErrorKind, ErrorType, SetDutyCycle};

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl pwm::Error for Error {
    fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl<'a> Arbitrary<'a> for Error {
    fn arbitrary(_: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Error {
            kind: ErrorKind::Other,
        })
    }
}

/// Creates a fuzzed Pwm driver, this type is intended to be constructed
/// the arbitrary crate e.g.
/// ```rust
/// use arbitrary::{Arbitrary, Unstructured};
/// use embedded_hal_fuzz::pwm::ArbitraryPwm;
/// let raw_fuzzed_data = &[1u8, 2, 3, 4, 5][..];
/// let mut unstructured = Unstructured::new(raw_fuzzed_data);
/// let spi_bus = ArbitraryPwm::arbitrary(&mut unstructured);
/// ```
#[derive(Debug, Arbitrary)]
pub struct ArbitraryPwm {
    max_duty_cycle: u16,
    maybe_error: Vec<Result<(), Error>>,
}

impl ErrorType for ArbitraryPwm {
    type Error = Error;
}

impl SetDutyCycle for ArbitraryPwm {
    fn get_max_duty_cycle(&self) -> u16 {
        self.max_duty_cycle
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle {
            return Err(Error {
                kind: ErrorKind::Other,
            });
        }
        if let Some(result) = self.maybe_error.pop() {
            return result;
        }
        Ok(())
    }
}
