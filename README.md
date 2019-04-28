stm32-hal
=========

_stm32-hal_ contains a multi device hardware abstraction on top of the
peripheral access API for the STMicro STM32 series microcontrollers.
More specificially, the following series are currently supported:

* STM32F0
* STM32F1
* STM32F2
* STM32F3
* STM32F4

The idea behind this crate is to gloss over the slight differences in the
various peripherals available on those MCUs so a HAL can be written for all
chips in that same family without having to cut and paste crates for every
single model.

This crate relies on Adam Greigs fantastic [stm32-rs][] series of crates to
provide appropriate register definitions and implements a partial set of the
[embedded-hal][] traits.

* adc::OneShot  (only for STM32F4)
* blocking::delay::DelayMs
* blocking::delay::DelayUs
* blocking::i2c::Read
* blocking::i2c::Write
* blocking::i2c::WriteRead
* digital::InputPin
* digital::OutputPin
* digital::ToggleableOutputPin
* serial::Read
* serial::Write
* spi::FullDuplex
* timer::CountDown
* timer::Periodic
* watchdog::Watchdog
* watchdog::WatchdogEnable
* Qei

This crate also provides a few more features that are not (yet ?)
[embedded-hal][] traits:

* RCC Clocks configuration
* Device electronic signature (UID, Flashsize, Vref and Temp calibration)

The selection of the MCU is done by feature gates, typically specified by board
support crates. Currently supported configurations are:

* stm32f0x0
* stm32f0x1
* stm32f0x2
* stm32f0x8
* stm32f100
* stm32f101
* stm32f102
* stm32f103
* stm32f105
* stm32f107
* stm32f205
* stm32f207
* stm32f215
* stm32f217
* stm32f301
* stm32f302
* stm32f303
* stm32f318
* stm32f328
* stm32f334
* stm32f358
* stm32f373
* stm32f378
* stm32f398
* stm32f401
* stm32f405
* stm32f407
* stm32f410
* stm32f411
* stm32f412
* stm32f413
* stm32f415
* stm32f417
* stm32f423
* stm32f427
* stm32f429
* stm32f437
* stm32f439
* stm32f446
* stm32f469
* stm32f479

The Cortex-M runtime can also be enabled using one of those feature, depending
on the targetted STM32 serie:

* stm32f0-rt
* stm32f1-rt
* stm32f2-rt
* stm32f3-rt
* stm32f4-rt

Collaboration on this crate is highly welcome as are pull requests!

This crate was forked off from the [stm32f4xx-hal][] crate.

[stm32-rs]: https://github.com/smt32-rs/stm32-rs
[stm32f4xx-hal]: https://github.com/smt32-rs/stm32f4xx-hal
[embedded-hal]: https://github.com/japaric/embedded-hal

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
