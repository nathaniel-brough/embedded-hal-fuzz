use crate::error::FuzzedError;
use crate::shared_data::FuzzData;
use core::marker::PhantomData;
use core::ops::{BitOr, Shl};
use embedded_hal::adc::{Channel, OneShot};

/// A fuzzed backend for the Adc OneShot trait
pub struct AdcFuzz<'a, E: FuzzedError<'a>> {
    data: FuzzData<'a>,
    _e: PhantomData<E>,
}

impl<'a, E: FuzzedError<'a>> AdcFuzz<'a, E> {
    pub fn new(data: FuzzData<'a>) -> Self {
        AdcFuzz {
            data,
            _e: PhantomData,
        }
    }
}

impl<'a, ADC, Word, Pin, E> OneShot<ADC, Word, Pin> for AdcFuzz<'a, E>
where
    Word: From<u8> + Shl<usize, Output = Word> + BitOr<Output = Word>,
    Pin: Channel<ADC>,
    E: FuzzedError<'a>,
{
    type Error = E;

    fn read(&mut self, _pin: &mut Pin) -> nb::Result<Word, Self::Error> {
        // Randomly create an error.
        let mut data = match self.data.iter.lock() {
            Ok(data) => data,
            Err(_) => return Err(nb::Error::Other(Default::default())),
        };
        if let Some(err) = E::new_err(&mut *data) {
            return Err(nb::Error::Other(err));
        }
        //conversion from u8 into word size
        let word_bytes = std::mem::size_of::<Word>();
        let mut value: Word = Word::from(0);
        for byte in 0..word_bytes {
            let data = data
                .next()
                .copied()
                .map(Word::from)
                .ok_or_else(|| nb::Error::Other(Default::default()))?;
            value = value | (data << (byte * 8));
        }
        Ok(value)
    }
}

//Pin is ignored, so provide a dummy implentation for Pin<()>
#[derive(Clone, Copy, Default)]
pub struct DummyPin {}
impl Channel<()> for DummyPin {
    type ID = ();
    fn channel() -> Self::ID {}
}
