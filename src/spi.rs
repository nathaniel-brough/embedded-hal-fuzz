//! To make use of this module you can simply pass
//! this in as a value from the fuzz_target macro e.g.
//! ```rust
//! use libfuzzer_sys::fuzz_target;
//! use embedded_hal_fuzz::spi::ArbitrarySpiBus;
//! use embedded_hal::spi::SpiBus;
//! use arbitrary::Arbitrary;
//!
//! fuzz_target!(|spi: ArbitrarySpiBus<u16>| {
//!   let mut spi = spi;
//!   let mut read = vec![0;10];
//!   let _ = spi.read(&mut read);
//! });
//! ```
use arbitrary::Arbitrary;
use embedded_hal::spi::{self, ErrorKind, ErrorType, SpiBus};

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl<'a> Arbitrary<'a> for Error {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        use ErrorKind::*;
        Ok(Error {
            kind: *u.choose(&[Overrun, ModeFault, FrameFormat, ChipSelectFault, Other][..])?,
        })
    }
}

impl spi::Error for Error {
    fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl<Word> ErrorType for ArbitrarySpiBus<Word> {
    type Error = Error;
}

#[derive(Debug, Arbitrary)]
enum Action {
    TryRead,
    ReturnError(Error),
}

/// Creates a fuzzed SpiBus, this type is intended to be constructed
/// the arbitrary crate e.g.
/// ```rust
/// use arbitrary::{Arbitrary, Unstructured};
/// use embedded_hal_fuzz::spi::ArbitrarySpiBus;
/// let raw_fuzzed_data = &[1u8, 2, 3, 4, 5][..];
/// let mut unstructured = Unstructured::new(raw_fuzzed_data);
/// let spi_bus = ArbitrarySpiBus::<u8>::arbitrary(&mut unstructured);
/// ```
#[derive(Debug, Arbitrary)]
pub struct ArbitrarySpiBus<Word> {
    read_words: Vec<Word>,
    actions: Vec<Action>,
}

impl<Word: Copy + 'static> SpiBus<Word> for ArbitrarySpiBus<Word> {
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        match self.actions.pop() {
            Some(Action::TryRead) => {
                for word in words.iter_mut() {
                    *word = self.read_words.pop().ok_or(Error {
                        kind: ErrorKind::Other,
                    })?;
                }
                Ok(())
            }
            Some(Action::ReturnError(error)) => Err(error),
            None => Err(Error {
                kind: ErrorKind::Other,
            }),
        }
    }
    fn write(&mut self, _words: &[Word]) -> Result<(), Self::Error> {
        match self.actions.pop() {
            Some(Action::ReturnError(error)) => Err(error),
            _ => Ok(()),
        }
    }

    fn transfer(&mut self, read: &mut [Word], _write: &[Word]) -> Result<(), Self::Error> {
        self.read(read)
    }
    fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        self.read(words)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        match self.actions.pop() {
            Some(Action::ReturnError(error)) => Err(error),
            _ => Ok(()),
        }
    }
}
