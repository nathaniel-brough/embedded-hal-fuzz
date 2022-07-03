use crate::error::FuzzedError;
use crate::shared_data::FuzzData;
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{AddressMode, Read, Write, WriteRead};

/// A fuzzed backend for the I2C read trait.
pub struct I2cFuzz<'a, E: FuzzedError<'a>> {
    data: FuzzData<'a>,
    _e: PhantomData<E>,
}

impl<'a, E: FuzzedError<'a>> I2cFuzz<'a, E> {
    pub fn new(data: FuzzData<'a>) -> Self {
        I2cFuzz {
            data,
            _e: PhantomData,
        }
    }
}

impl<'a, A: AddressMode, E: FuzzedError<'a>> Read<A> for I2cFuzz<'a, E> {
    type Error = E;
    fn read(&mut self, _address: A, buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Randomly create an error.
        let mut data = match self.data.iter.lock() {
            Ok(data) => data,
            Err(_) => return Err(Default::default()),
        };
        if let Some(err) = E::new_err(&mut *data) {
            return Err(err);
        }
        for element in buffer.iter_mut() {
            *element = *data.next().ok_or(Default::default())?;
        }
        Ok(())
    }
}

impl<'a, A: AddressMode, E: FuzzedError<'a>> WriteRead<A> for I2cFuzz<'a, E> {
    type Error = E;
    fn write_read(
        &mut self,
        address: A,
        _ignore_write_buffer: &[u8],
        read_buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.read(address, read_buffer)
    }
}

impl<'a, A: AddressMode, E: FuzzedError<'a>> Write<A> for I2cFuzz<'a, E> {
    type Error = E;
    fn write(
        &mut self,
        address: A,
        _ignore_write_buffer: &[u8],
    ) -> Result<(), Self::Error> {
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