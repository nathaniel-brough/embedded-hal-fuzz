//! To make use of this module you can simply pass
//! this in as a value from the fuzz_target macro e.g.
//!
//! # Example
//!
//! ```rust
//! use libfuzzer_sys::fuzz_target;
//! use embedded_hal_fuzz::i2c::ArbitraryI2c;
//! use embedded_hal::i2c::{I2c, SevenBitAddress, TenBitAddress};
//!
//! fuzz_target!(|i2c: ArbitraryI2c<SevenBitAddress>| {
//!   let mut i2c = i2c;
//!   let mut read = vec![0;10];
//!   let _ = i2c.read(1, &mut read);
//! });
//! ```

use arbitrary::Arbitrary;
use embedded_hal::i2c::{
    self, ErrorKind, I2c, NoAcknowledgeSource, Operation, SevenBitAddress, TenBitAddress,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Default for Error {
    fn default() -> Self {
        Error {
            kind: ErrorKind::Other,
        }
    }
}

impl i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl<'a> Arbitrary<'a> for Error {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        use ErrorKind::*;
        use NoAcknowledgeSource::*;
        Ok(Error {
            kind: *u.choose(&[
                Bus,
                ArbitrationLoss,
                NoAcknowledge(Address),
                NoAcknowledge(Data),
                NoAcknowledge(Unknown),
                Overrun,
                Other,
            ])?,
        })
    }
}

/// Creates a fuzzed i2c driver, this type is intended to be constructed
/// the arbitrary crate e.g.
/// ```rust
/// use arbitrary::{Arbitrary, Unstructured};
/// use embedded_hal_fuzz::i2c::ArbitraryI2c;
/// use embedded_hal::i2c::{SevenBitAddress, TenBitAddress};
/// let raw_fuzzed_data = &[1u8, 2, 3, 4, 5][..];
/// let mut unstructured = Unstructured::new(raw_fuzzed_data);
/// let seven_bit_spi_bus = ArbitraryI2c::<SevenBitAddress>::arbitrary(&mut unstructured);
/// let ten_bit_spi_bus = ArbitraryI2c::<TenBitAddress>::arbitrary(&mut unstructured);
/// ```
#[derive(Debug, Arbitrary)]
pub struct ArbitraryI2c<T> {
    read_data: Vec<u8>,
    maybe_error: Vec<Result<(), Error>>,
    _p: PhantomData<T>,
}

impl<T> ArbitraryI2c<T> {
    fn transaction_impl(&mut self, operations: &mut [Operation<'_>]) -> Result<(), Error> {
        self.maybe_error.pop().ok_or(Error::default())??;
        let operation_result: Result<Vec<_>, _> = operations
            .iter_mut()
            .map(|x| match x {
                Operation::Read(read) => {
                    let result: Result<Vec<_>, Error> = read
                        .iter_mut()
                        .map(|x| {
                            *x = self.read_data.pop().ok_or(Error::default())?;
                            Ok(())
                        })
                        .collect();
                    result
                }
                Operation::Write(_) => Ok(Vec::new()),
            })
            .collect();
        operation_result.map(|_| ())
    }
}

impl<T> i2c::ErrorType for ArbitraryI2c<T> {
    type Error = Error;
}

impl I2c<SevenBitAddress> for ArbitraryI2c<SevenBitAddress> {
    fn transaction(
        &mut self,
        _address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.transaction_impl(operations)
    }
}

impl I2c<TenBitAddress> for ArbitraryI2c<TenBitAddress> {
    fn transaction(
        &mut self,
        _address: u16,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.transaction_impl(operations)
    }
}
