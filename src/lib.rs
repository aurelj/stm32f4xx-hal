#![no_std]
#![allow(non_camel_case_types)]

#[cfg(not(feature = "device-selected"))]
compile_error!("This crate requires one of the following device features enabled:
        stm32f0x0
        stm32f0x1
        stm32f0x2
        stm32f0x8
        stm32f100
        stm32f101
        stm32f102
        stm32f103
        stm32f105
        stm32f107
        stm32f205
        stm32f207
        stm32f215
        stm32f217
        stm32f401
        stm32f405
        stm32f407
        stm32f410
        stm32f411
        stm32f412
        stm32f413
        stm32f415
        stm32f417
        stm32f423
        stm32f427
        stm32f429
        stm32f437
        stm32f439
        stm32f446
        stm32f469
        stm32f479");

pub use embedded_hal as hal;

pub use nb;
pub use nb::block;

#[cfg(feature = "stm32f0x0")]
pub use stm32f0::stm32f0x0 as stm32;

#[cfg(feature = "stm32f0x1")]
pub use stm32f0::stm32f0x1 as stm32;

#[cfg(feature = "stm32f0x2")]
pub use stm32f0::stm32f0x2 as stm32;

#[cfg(feature = "stm32f0x8")]
pub use stm32f0::stm32f0x8 as stm32;

#[cfg(feature = "stm32f100")]
pub use stm32f1::stm32f100 as stm32;

#[cfg(feature = "stm32f101")]
pub use stm32f1::stm32f101 as stm32;

#[cfg(feature = "stm32f102")]
pub use stm32f1::stm32f102 as stm32;

#[cfg(feature = "stm32f103")]
pub use stm32f1::stm32f103 as stm32;

#[cfg(feature = "stm32f105")]
pub use stm32f1::stm32f107 as stm32;

#[cfg(feature = "stm32f107")]
pub use stm32f1::stm32f107 as stm32;

#[cfg(feature = "stm32f205")]
pub use stm32f2::stm32f215 as stm32;

#[cfg(feature = "stm32f207")]
pub use stm32f2::stm32f217 as stm32;

#[cfg(feature = "stm32f215")]
pub use stm32f2::stm32f215 as stm32;

#[cfg(feature = "stm32f217")]
pub use stm32f2::stm32f217 as stm32;

#[cfg(feature = "stm32f401")]
pub use stm32f4::stm32f401 as stm32;

#[cfg(feature = "stm32f405")]
pub use stm32f4::stm32f405 as stm32;

#[cfg(feature = "stm32f407")]
pub use stm32f4::stm32f407 as stm32;

#[cfg(feature = "stm32f410")]
pub use stm32f4::stm32f410 as stm32;

#[cfg(feature = "stm32f411")]
pub use stm32f4::stm32f411 as stm32;

#[cfg(feature = "stm32f412")]
pub use stm32f4::stm32f412 as stm32;

#[cfg(feature = "stm32f413")]
pub use stm32f4::stm32f413 as stm32;

#[cfg(feature = "stm32f415")]
pub use stm32f4::stm32f405 as stm32;

#[cfg(feature = "stm32f417")]
pub use stm32f4::stm32f407 as stm32;

#[cfg(feature = "stm32f423")]
pub use stm32f4::stm32f413 as stm32;

#[cfg(feature = "stm32f427")]
pub use stm32f4::stm32f427 as stm32;

#[cfg(feature = "stm32f429")]
pub use stm32f4::stm32f429 as stm32;

#[cfg(feature = "stm32f437")]
pub use stm32f4::stm32f427 as stm32;

#[cfg(feature = "stm32f439")]
pub use stm32f4::stm32f429 as stm32;

#[cfg(feature = "stm32f446")]
pub use stm32f4::stm32f446 as stm32;

#[cfg(feature = "stm32f469")]
pub use stm32f4::stm32f469 as stm32;

#[cfg(feature = "stm32f479")]
pub use stm32f4::stm32f469 as stm32;

// Enable use of interrupt macro
#[cfg(any(
    feature = "stm32f0-rt",
    feature = "stm32f1-rt",
    feature = "stm32f2-rt",
    feature = "stm32f4-rt",
))]
pub use crate::stm32::interrupt;

#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f2", feature = "stm32f4"))]
pub mod delay;
#[cfg(feature = "stm32f1")]
pub mod gpio_v1;
#[cfg(feature = "stm32f1")]
pub use crate::gpio_v1 as gpio;
#[cfg(any(feature = "stm32f0", feature = "stm32f2", feature = "stm32f4"))]
pub mod gpio;
#[cfg(any(feature = "stm32f1", feature = "stm32f2", feature = "stm32f4"))]
pub mod i2c;
#[cfg(feature = "stm32f0")]
pub mod i2c_v2;
#[cfg(feature = "stm32f0")]
pub use crate::i2c_v2 as i2c;
#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f2", feature = "stm32f4"))]
pub mod prelude;
#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f2", feature = "stm32f4"))]
pub mod rcc;
#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f4"))]
pub mod serial;
#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f2", feature = "stm32f4"))]
pub mod spi;
#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f2", feature = "stm32f4"))]
pub mod time;
#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f2", feature = "stm32f4"))]
pub mod timer;
#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f4"))]
pub mod qei;
#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f4"))]
pub mod watchdog;
#[cfg(feature = "stm32f4")]
pub mod adc;
#[cfg(any(feature = "stm32f0", feature = "stm32f1", feature = "stm32f4"))]
pub mod signature;
