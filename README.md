[![Rust](https://github.com/silvergasp/embedded-hal-fuzz/actions/workflows/rust.yml/badge.svg)](https://github.com/silvergasp/embedded-hal-fuzz/actions/workflows/rust.yml)
[![Crates](https://img.shields.io/crates/v/embedded_hal_fuzz)](https://crates.io/crates/embedded-hal-fuzz)

<p align="center">
  <img src="logo.png" />
</p>


# embedded-hal-fuzz
A fuzzing library for rust hardware drivers. Checkout the [docs](https://docs.rs/embedded-hal-fuzz/1.0.0-rc.1/embedded_hal_fuzz/) for more details.

## Feature status

- [x] I²C (Read, WriteRead)
- [x] SPI (FullDuplex, Write, WriteIter, Transactional)
- [x] No-op delay
- [x] Serial
- [x] I/O pins (InputPin, OutputPin)
- [x] PWM

