//! To make use of this module you can simply pass
//! this in as a value from the fuzz_target macro e.g.
//! ```rust
//! use libfuzzer_sys::fuzz_target;
//! use embedded_hal_fuzz::digital::{ArbitraryInputPin, ArbitraryOutputPin};
//! use embedded_hal::digital::{InputPin, OutputPin};
//! use arbitrary::Arbitrary;
//! #[derive(Debug, Arbitrary)]
//! struct Ctx {
//!   input_pin : ArbitraryInputPin,
//!   output_pin : ArbitraryOutputPin,
//! }
//! fuzz_target!(|ctx: Ctx| {
//!   let Ctx {input_pin, mut output_pin } = ctx;
//!   let _ = output_pin.set_high();
//! });
//! ```
use std::cell::RefCell;

use arbitrary::{Arbitrary, Unstructured};
use embedded_hal::digital::{
    self, ErrorKind, ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin,
};

#[derive(Debug, Arbitrary)]
pub struct Error;

impl digital::Error for Error {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

/// Creates a fuzzed input pin driver, this type is intended to be constructed
/// the arbitrary crate e.g.
/// ```rust
/// use arbitrary::{Arbitrary, Unstructured};
/// use embedded_hal_fuzz::digital::ArbitraryInputPin;
/// let raw_fuzzed_data = &[1u8, 2, 3, 4, 5][..];
/// let mut u = Unstructured::new(raw_fuzzed_data);
/// let arbitrary_input_pin = ArbitraryInputPin::arbitrary(&mut u);
/// ```
#[derive(Debug, Arbitrary)]
pub struct ArbitraryInputPin {
    pin_states: RefCell<Vec<Result<bool, Error>>>,
}

impl ErrorType for ArbitraryInputPin {
    type Error = Error;
}

impl InputPin for ArbitraryInputPin {
    fn is_high(&self) -> Result<bool, Self::Error> {
        match self.pin_states.try_borrow_mut() {
            Ok(mut pin_states) => match pin_states.pop() {
                Some(result) => result,
                None => Err(Error),
            },
            Err(_) => Err(Error),
        }
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_high().map(|x| !x)
    }
}

/// Creates a fuzzed output pin driver, this type is intended to be constructed
/// the arbitrary crate e.g.
/// ```rust
/// use arbitrary::{Arbitrary, Unstructured};
/// use embedded_hal_fuzz::digital::ArbitraryOutputPin;
/// let raw_fuzzed_data = &[1u8, 2, 3, 4, 5][..];
/// let mut u = Unstructured::new(raw_fuzzed_data);
/// let arbitrary_output_pin = ArbitraryOutputPin::arbitrary(&mut u);
/// ```
#[derive(Debug, Arbitrary)]
pub struct ArbitraryOutputPin {
    maybe_error: RefCell<Vec<Option<Error>>>,
    #[arbitrary(with = |u: &mut Unstructured| Ok(if bool::arbitrary(u)? {PinState::High} else {PinState::Low}))]
    state: PinState,
}

impl OutputPin for ArbitraryOutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        match self.maybe_error.try_borrow_mut().map_err(|_| Error)?.pop() {
            Some(Some(error)) => Err(error),
            _ => {
                self.state = PinState::Low;
                Ok(())
            }
        }
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        match self.maybe_error.try_borrow_mut().map_err(|_| Error)?.pop() {
            Some(Some(error)) => Err(error),
            _ => {
                self.state = PinState::High;
                Ok(())
            }
        }
    }
}

impl StatefulOutputPin for ArbitraryOutputPin {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        match self.maybe_error.try_borrow_mut().map_err(|_| Error)?.pop() {
            Some(Some(error)) => Err(error),
            _ => Ok(self.state == PinState::High),
        }
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        match self.maybe_error.try_borrow_mut().map_err(|_| Error)?.pop() {
            Some(Some(error)) => Err(error),
            _ => Ok(self.state == PinState::Low),
        }
    }
}

impl ErrorType for ArbitraryOutputPin {
    type Error = Error;
}
