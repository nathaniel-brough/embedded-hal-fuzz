#![no_main]
/// This fuzz test should fail.
use embedded_hal::adc::OneShot;
use embedded_hal_fuzz as hal_fuzz;
use hal_fuzz::adc::DummyPin;
use libfuzzer_sys::fuzz_target;

struct DodgyDriver<T: OneShot<(), u16, DummyPin>> {
    adc: T,
}

impl<T: OneShot<(), u16, DummyPin>> DodgyDriver<T> {
    fn new(adc: T) -> Self {
        Self { adc }
    }

    fn get_dodgy_average_value(&mut self) -> Result<u16, ()> {
        let mut average = 0;
        for _ in 0..10 {
            // May overflow.
            average +=
                self.adc.read(&mut DummyPin::default()).map_err(|_| ())?;
        }
        Ok(average / 10)
    }
}

type AdcError = ();

fuzz_target!(|data: &[u8]| {
    // Ignore empty inputs.
    if data.len() > 0 {
        use hal_fuzz::shared_data::FuzzData;
        let data = FuzzData::new(data);
        let adc: hal_fuzz::adc::AdcFuzz<'_, AdcError> =
            hal_fuzz::adc::AdcFuzz::new(data);
        let mut driver = DodgyDriver::new(adc);
        let _ = driver.get_dodgy_average_value();
    }
});
