#![no_main]
/// This fuzz test should pass.
use embedded_hal::adc::OneShot;
use embedded_hal_fuzz as hal_fuzz;
use hal_fuzz::adc::DummyPin;
use libfuzzer_sys::fuzz_target;

struct Driver<T: OneShot<(), u16, DummyPin>> {
    adc: T,
}

impl<T: OneShot<(), u16, DummyPin>> Driver<T> {
    fn new(adc: T) -> Self {
        Self { adc }
    }

    fn get_average_value(&mut self) -> Result<u16, ()> {
        //never overflow U16::MAX * 10 < U32::MAX
        let mut average: u32 = 0;
        for _ in 0..10 {
            average +=
                self.adc.read(&mut DummyPin::default()).map_err(|_| ())? as u32;
        }
        Ok((average / 10) as u16)
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
        let mut driver = Driver::new(adc);
        let _ = driver.get_average_value();
    }
});
