use crate::error::FuzzedError;
use crate::shared_data::FuzzData;
use core::marker::PhantomData;
use embedded_hal::digital::v2::{InputPin, OutputPin};

/// A fuzzed backend for the digital input trait.
pub struct InputPinFuzz<'a, E: FuzzedError<'a>> {
    data: FuzzData<'a>,
    _e: PhantomData<E>,
}

impl<'a, E: FuzzedError<'a>> InputPinFuzz<'a, E> {
    pub fn new(data: FuzzData<'a>) -> Self {
        InputPinFuzz {
            data,
            _e: PhantomData,
        }
    }

    fn pin_state(&self) -> Result<bool, E> {
        // Randomly create an error.
        let mut data = match self.data.iter.lock() {
            Ok(data) => data,
            Err(_) => return Err(Default::default()),
        };
        if let Some(err) = E::new_err(&mut *data) {
            return Err(err);
        }
        let byte = data.next();
        match byte {
            Some(x) if *x > 128 => Ok(false),
            _ => Ok(true),
        }
    }
}

impl<'a, E: FuzzedError<'a>> InputPin for InputPinFuzz<'a, E> {
    type Error = E;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.pin_state()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.pin_state()
    }
}

/// A fuzzed backend for the digital input trait.
pub struct OutputPinFuzz<'a, E: FuzzedError<'a>> {
    data: FuzzData<'a>,
    _e: PhantomData<E>,
}

impl<'a, E: FuzzedError<'a>> OutputPinFuzz<'a, E> {
    pub fn new(data: FuzzData<'a>) -> Self {
        OutputPinFuzz {
            data,
            _e: PhantomData,
        }
    }

    fn potential_error(&self) -> Result<(), E> {
        // Randomly create an error.
        let mut data = match self.data.iter.lock() {
            Ok(data) => data,
            Err(_) => return Err(Default::default()),
        };
        if let Some(err) = E::new_err(&mut *data) {
            return Err(err);
        }
        Ok(())
    }
}

impl<'a, E: FuzzedError<'a>> OutputPin for OutputPinFuzz<'a, E> {
    type Error = E;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        // set_low is an output, so we ignore it here.
        // Potentially create an error.
        self.potential_error()
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        // set_high is an output, so we ignore it here.
        // Potentially create an error.
        self.potential_error()
    }
}
