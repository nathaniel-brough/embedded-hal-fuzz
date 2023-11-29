#![no_main]
/// This fuzz test should fail.
use embedded_hal::i2c::{I2c, SevenBitAddress};
use embedded_hal_fuzz as hal_fuzz;
use libfuzzer_sys::fuzz_target;

struct DodgyDriver<T> {
    i2c: T,
}

impl<T: I2c> DodgyDriver<T> {
    fn new(i2c: T) -> Self {
        Self { i2c }
    }

    fn get_dodgy_scaled_value(&mut self) -> Result<u8, ()> {
        let mut buffer = [0u8; 1];
        self.i2c.read(0x01, &mut buffer).map_err(|_| ())?;
        let a = buffer[0];
        // NOTE: Fuzzing will ignore the write buffer as it's not an input.
        self.i2c
            .write_read(0x01, &[1, 2, 3, 4], &mut buffer)
            .map_err(|_| ())?;
        // May overflow.
        Ok(buffer[0] * a)
    }
}

fuzz_target!(|i2c: hal_fuzz::i2c::ArbitraryI2c<SevenBitAddress>| {
    // Ignore empty inputs.
    let mut driver = DodgyDriver::new(i2c);
    let _ = driver.get_dodgy_scaled_value();
});
