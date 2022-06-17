#![no_main]
/// This fuzz test should pass.
use embedded_hal::digital::v2::InputPin;
use embedded_hal::spi::FullDuplex;
use embedded_hal_fuzz as hal_fuzz;
use libfuzzer_sys::fuzz_target;

struct Driver<T: FullDuplex<u8>, P: InputPin> {
    spi: T,
    pin: P,
}

impl<T: FullDuplex<u8>, P: InputPin> Driver<T, P> {
    fn new(spi: T, pin: P) -> Self {
        Self { spi, pin }
    }

    fn get_scaled_value(&mut self) -> Result<f32, ()> {
        if !self.pin.is_high().map_err(|_| ())? {
            return Ok(0.0);
        }
        self.spi.send(0x01).map_err(|_| ())?;
        let a = self.spi.read().map_err(|_| ())?;
        Ok(a as f32)
    }
}

type Error = ();

fuzz_target!(|data: &[u8]| {
    // Ignore empty inputs.
    if data.len() > 0 {
        use hal_fuzz::shared_data::FuzzData;
        let data = FuzzData::new(data);
        let spi: hal_fuzz::spi::SpiFuzz<'_, Error> = hal_fuzz::spi::SpiFuzz::new(data.clone());
        let pin: hal_fuzz::digital::InputPinFuzz<'_, Error> =
            hal_fuzz::digital::InputPinFuzz::new(data.clone());
        let mut driver = Driver::new(spi, pin);
        let _ = driver.get_scaled_value();
    }
});
