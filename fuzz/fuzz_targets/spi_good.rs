#![no_main]
/// This fuzz test should pass.
use embedded_hal::spi::FullDuplex;
use embedded_hal_fuzz as hal_fuzz;
use libfuzzer_sys::fuzz_target;

struct GoodDriver<T: FullDuplex<u8>> {
    spi: T,
}

impl<T: FullDuplex<u8>> GoodDriver<T> {
    fn new(spi: T) -> Self {
        Self { spi }
    }

    fn get_scaled_value(&mut self) -> Result<f32, ()> {
        self.spi.send(0x01).map_err(|_| ())?;
        let a = self.spi.read().map_err(|_| ())?;
        Ok(a as f32)
    }
}

type I2cError = ();

fuzz_target!(|data: &[u8]| {
    // Ignore empty inputs.
    if data.len() > 0 {
        use hal_fuzz::shared_data::FuzzData;
        let data = FuzzData::new(data);
        let spi: hal_fuzz::spi::SpiFuzz<'_, I2cError> = hal_fuzz::spi::SpiFuzz::new(data);
        let mut driver = GoodDriver::new(spi);
        let _ = driver.get_scaled_value();
    }
});
