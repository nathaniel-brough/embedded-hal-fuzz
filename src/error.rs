/// Optionally creates a new error from a byte iterator.
///
/// When implementing this trait it is important to consider the probability of
/// creating an error or not. For example if new_err always returns an error,
/// your library will only be tested as if it always fails. The opposite is true
/// for an implementation that never returns an error.
///
/// As a general rule of thumb, this trait should be implemented to return an
/// error ~50% of the time.
///
/// # Example
///
/// Here we will implement the fuzzed error type for an enum.
///
/// ```rust
/// use embedded_hal_fuzz::error::FuzzedError;
///
/// // Required for example only.
/// use num_derive::FromPrimitive;
/// use num_traits::FromPrimitive;
///
/// #[derive(FromPrimitive)]
/// enum MyError {
///    Foo = 0,
///    Bar = 1,
/// }
///
/// impl Default for MyError {
///    fn default() -> Self {
///       MyError::Foo
///    }
/// }
///
/// impl<'a> FuzzedError<'a> for MyError {
///    fn new_err(data: &mut impl Iterator<Item=&'a u8>) -> Option<Self> {
///        FromPrimitive::from_u8(*data.next()?)
///    }
/// }
/// ```

pub trait FuzzedError<'a>: Default {
    fn new_err(data: &mut impl Iterator<Item = &'a u8>) -> Option<Self>
    where
        Self: Sized;
}

/// In the case that you only care about an error/success you can use this error
/// implementation which will return a unit type error ~50% of the time.
impl<'a> FuzzedError<'a> for () {
    fn new_err(data: &mut impl Iterator<Item = &'a u8>) -> Option<Self> {
        if let Some(byte) = data.next() {
            if *byte & 0x01 == 0 {
                return Some(());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_error() {
        let average_errors = (0..255u8)
            .map(|x| <()>::new_err(&mut [x.into()].iter()))
            .filter(|x| x.is_some())
            .count();
        assert_eq!(average_errors, 255 / 2 + 1);
    }
}
