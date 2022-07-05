use crate::error::FuzzedError;
use crate::shared_data::FuzzData;
use core::marker::PhantomData;
use embedded_hal::spi::FullDuplex;
use nb::Result;

pub struct SpiFuzz<'a, E> {
    data: FuzzData<'a>,
    e_: PhantomData<E>,
}

impl<'a, E: FuzzedError<'a>> SpiFuzz<'a, E> {
    pub fn new(data: FuzzData<'a>) -> Self {
        SpiFuzz {
            data,
            e_: PhantomData,
        }
    }
}

// Use default implementations for Write, WriteIter, Transfer
impl<'a, E: FuzzedError<'a>> embedded_hal::blocking::spi::write_iter::Default<u8>
    for SpiFuzz<'a, E>
{
}
impl<'a, E: FuzzedError<'a>> embedded_hal::blocking::spi::write::Default<u8> for SpiFuzz<'a, E> {}
impl<'a, E: FuzzedError<'a>> embedded_hal::blocking::spi::transfer::Default<u8> for SpiFuzz<'a, E> {}

impl<'a, E: FuzzedError<'a>> FullDuplex<u8> for SpiFuzz<'a, E> {
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
        data.next().copied().ok_or(nb::Error::WouldBlock)
    }

    fn send(&mut self, _: u8) -> Result<(), Self::Error> {
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
