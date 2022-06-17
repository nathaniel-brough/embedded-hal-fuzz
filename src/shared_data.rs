/// The purpose of this module is to provide a common base for
/// implementing a shared fuzzed backend for the various traits.
///
/// The reasoning for this is to ensure that each call to a HAL
/// trait corresponds to a unique point in the fuzzing data. e.g.
/// The fuzzed result from an InputPin API should pull from the
/// same set of data as a call to the SPI API. With each call
/// mapping to a unique point in the fuzzing data.
///
/// This is important to ensure that the fuzzing engine has the
/// ability to individually control the fuzzing of each trait,
/// maximising code coverage.
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct FuzzData<'a> {
    pub(crate) iter: Arc<Mutex<std::slice::Iter<'a, u8>>>,
}

impl<'a> FuzzData<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            iter: Arc::new(Mutex::new(data.iter())),
        }
    }
}
