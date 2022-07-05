use crate::error::FuzzedError;
use crate::shared_data::FuzzData;
use core::marker::PhantomData;
use embedded_hal::serial::{Read, Write};
use nb::Result;

pub struct SerialFuzz<'a, E> {
    data: FuzzData<'a>,
    e_: PhantomData<E>,
}

impl<'a, E: FuzzedError<'a>> embedded_hal::blocking::serial::write::Default<u8>
    for SerialFuzz<'a, E>
{
}

impl<'a, E: FuzzedError<'a>> SerialFuzz<'a, E> {
    pub fn new(data: FuzzData<'a>) -> Self {
        SerialFuzz {
            data,
            e_: PhantomData,
        }
    }

    fn maybe_error(&mut self) -> Result<(), E> {
        let mut data = match self.data.iter.lock() {
            Ok(data) => data,
            Err(_) => return Err(nb::Error::WouldBlock),
        };
        // Create an error/or not based on fuzzed data.
        if let Some(err) = E::new_err(&mut *data) {
            return Err(nb::Error::Other(err));
        }
        // NOTE: We ignore the value sent as it is an output and therefore
        //       not relevant for fuzzing.
        Ok(())
    }
}

impl<'a, E: FuzzedError<'a>> Read<u8> for SerialFuzz<'a, E> {
    type Error = E;

    fn read(&mut self) -> Result<u8, Self::Error> {
        // Randomly create an error.
        let mut data = match self.data.iter.lock() {
            Ok(data) => data,
            Err(_) => return Err(nb::Error::WouldBlock),
        };
        if let Some(err) = E::new_err(&mut *data) {
            return Err(nb::Error::Other(err));
        }
        // TODO: Explore the possibility of having a fuzzed error here.
        // it's possible that have a WouldBlock may result in a forever loop in
        // the fuzzer.
        data.next().map(|x| *x).ok_or(nb::Error::WouldBlock)
    }
}

impl<'a, E: FuzzedError<'a>> Write<u8> for SerialFuzz<'a, E> {
    type Error = E;

    fn write(&mut self, _: u8) -> Result<(), Self::Error> {
        self.maybe_error()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.maybe_error()
    }
}
