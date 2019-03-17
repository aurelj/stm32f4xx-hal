use core::ptr;
use core::ops::Deref;

use embedded_hal::spi;
pub use embedded_hal::spi::{Mode, Phase, Polarity};
use nb;

use crate::stm32::{spi1, RCC};

#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f328",
    feature = "stm32f334",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::SPI1;

#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::SPI2;

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f104",
    feature = "stm32f105",
    feature = "stm32f107",
    feature = "stm32f2",
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::SPI3;

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::SPI4;

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::SPI5;

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::SPI6;

#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpioa::PA1;
#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
use crate::gpio::gpioa::{PA2, PA3, PA8, PA13};
#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioa::PA9;
#[cfg(any(
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpioa::PA10;
#[cfg(any(
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f398",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpioa::PA11;
#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpioa::PA12;
#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f328",
    feature = "stm32f334",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioa::{PA5, PA6, PA7};

#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
use crate::gpio::gpiob::PB0;
#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpiob::PB12;
#[cfg(any(feature = "stm32f446"))]
use crate::gpio::gpiob::PB2;
#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpiob::PB8;
#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f3",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiob::{PB3, PB4, PB5};
#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f358",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiob::{PB13, PB14, PB15};
#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
use crate::gpio::gpiob::{PB14, PB15};
#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f2",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiob::PB10;

#[cfg(any(
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioc::PC1;
#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
use crate::gpio::gpioc::PC7;
#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
use crate::gpio::gpioc::{PC8, PC9};
#[cfg(any(
    feature = "stm32f105",
    feature = "stm32f107",
    feature = "stm32f2",
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioc::{PC10, PC11, PC12};
#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f2",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioc::{PC2, PC3};

#[cfg(any(feature = "stm32f446"))]
use crate::gpio::gpiod::PD0;
#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
use crate::gpio::gpiod::PD1;
#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8",
    feature = "stm32f373",
    feature = "stm32f378"
))]
use crate::gpio::gpiod::{PD3, PD4};
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiod::{PD3, PD6};
#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
use crate::gpio::gpiod::{PD7, PD8};

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioe::{PE12, PE13, PE14, PE2, PE5, PE6};
#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
use crate::gpio::gpioe::{PE13, PE14, PE15};

#[cfg(any(
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f398"
))]
use crate::gpio::gpiof::PF1;
#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
use crate::gpio::gpiof::PF6;
#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiof::{PF11, PF7, PF8, PF9};
#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
use crate::gpio::gpiof::{PF9, PF10};

#[cfg(any(feature = "stm32f446"))]
use crate::gpio::gpiog::PG11;
#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiog::PG14;
#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiog::{PG12, PG13};

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioh::{PH6, PH7};

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioi::{PI1, PI2, PI3};

#[cfg(feature = "stm32f0")]
use crate::gpio::{AF0, AF1, AF5};
#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::{AF5, AF6};
#[cfg(any(
    feature = "stm32f328",
    feature = "stm32f334"
))]
use crate::gpio::AF5;
#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
use crate::gpio::AF7;
#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f3",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::Alternate;
#[cfg(feature = "stm32f1")]
use crate::gpio::{Input, Floating, PushPull};

use crate::rcc::Clocks;
use crate::time::Hertz;

/// SPI error
#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
    #[doc(hidden)]
    _Extensible,
}

pub trait Pins<SPI> {}
pub trait PinSck<SPI> {}
pub trait PinMiso<SPI> {}
pub trait PinMosi<SPI> {}

impl<SPI, SCK, MISO, MOSI> Pins<SPI> for (SCK, MISO, MOSI)
where
    SCK: PinSck<SPI>,
    MISO: PinMiso<SPI>,
    MOSI: PinMosi<SPI>,
{}

/// A filler type for when the SCK pin is unnecessary
pub struct NoSck;
/// A filler type for when the Miso pin is unnecessary
pub struct NoMiso;
/// A filler type for when the Mosi pin is unnecessary
pub struct NoMosi;

macro_rules! pins {
    ($($SPIX:ty: SCK: [$($SCK:ty),*] MISO: [$($MISO:ty),*] MOSI: [$($MOSI:ty),*])+) => {
        $(
            $(
                impl PinSck<$SPIX> for $SCK {}
            )*
            $(
                impl PinMiso<$SPIX> for $MISO {}
            )*
            $(
                impl PinMosi<$SPIX> for $MOSI {}
            )*
        )+
    }
}

#[cfg(feature = "stm32f0")]
pins! {
    SPI1:
        SCK: [
            NoSck,
            PA5<Alternate<AF0>>,
            PB3<Alternate<AF0>>
        ]
        MISO: [
            NoMiso,
            PA6<Alternate<AF0>>,
            PB4<Alternate<AF0>>
        ]
        MOSI: [
            NoMosi,
            PA7<Alternate<AF0>>,
            PB5<Alternate<AF0>>
        ]

    SPI2:
        SCK: [
            NoSck,
            PB10<Alternate<AF5>>,
            PB13<Alternate<AF0>>
        ]
        MISO: [
            NoMiso,
            PB14<Alternate<AF0>>,
            PC2<Alternate<AF1>>
        ]
        MOSI: [
            NoMosi,
            PB15<Alternate<AF0>>,
            PC3<Alternate<AF1>>
        ]
}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
pins! {
    SPI1:
        SCK: [
            PE13<Alternate<AF1>>
        ]
        MISO: [
            PE14<Alternate<AF1>>
        ]
        MOSI: [
            PE15<Alternate<AF1>>
        ]

    SPI2:
        SCK: [
            PD1<Alternate<AF1>>
        ]
        MISO: [
            PD3<Alternate<AF1>>
        ]
        MOSI: [
            PD4<Alternate<AF1>>
        ]
}

#[cfg(feature = "stm32f1")]
pins! {
    SPI1:
        SCK: [
            NoSck,
            PA5<Alternate<PushPull>>,
            PB3<Alternate<PushPull>>
        ]
        MISO: [
            NoMiso,
            PA6<Input<Floating>>,
            PB4<Input<Floating>>
        ]
        MOSI: [
            NoMosi,
            PA7<Alternate<PushPull>>,
            PB5<Alternate<PushPull>>
        ]

    SPI2:
        SCK: [
            NoSck,
            PB13<Alternate<PushPull>>
        ]
        MISO: [
            NoMiso,
            PB14<Input<Floating>>
        ]
        MOSI: [
            NoMosi,
            PB15<Alternate<PushPull>>
        ]
}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f328",
    feature = "stm32f334",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI1:
        SCK: [
            NoSck,
            PA5<Alternate<AF5>>,
            PB3<Alternate<AF5>>
        ]
        MISO: [
            NoMiso,
            PA6<Alternate<AF5>>,
            PB4<Alternate<AF5>>
        ]
        MOSI: [
            NoMosi,
            PA7<Alternate<AF5>>,
            PB5<Alternate<AF5>>
        ]
}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f358",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI2:
        SCK: [
            NoSck,
            PB13<Alternate<AF5>>
        ]
        MISO: [
            NoMiso,
            PB14<Alternate<AF5>>
        ]
        MOSI: [
            NoMosi,
            PB15<Alternate<AF5>>
        ]
}

#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
pins! {
    SPI1:
        SCK: [
            PA12<Alternate<AF6>>,
            PC7<Alternate<AF5>>
        ]
        MISO: [
            PA13<Alternate<AF6>>,
            PC8<Alternate<AF5>>
        ]
        MOSI: [
            PB0<Alternate<AF5>>,
            PC9<Alternate<AF5>>,
            PF6<Alternate<AF5>>
        ]

    SPI2:
        SCK: [
            NoSck,
            PA8<Alternate<AF5>>,
            PB8<Alternate<AF5>>,
            PD7<Alternate<AF5>>,
            PD8<Alternate<AF5>>
        ]
        MISO: [
            NoMiso,
            PA9<Alternate<AF5>>,
            PB14<Alternate<AF5>>,
            PD3<Alternate<AF5>>
        ]
        MOSI: [
            NoMosi,
            PA10<Alternate<AF5>>,
            PB15<Alternate<AF5>>,
            PD4<Alternate<AF5>>
        ]
}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI2:
        SCK: [
            PB10<Alternate<AF5>>
        ]
        MISO: [
            PC2<Alternate<AF5>>
        ]
        MOSI: [
            PC3<Alternate<AF5>>
        ]
}

#[cfg(any(
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f398",
))]
pins! {
    SPI2:
        SCK: [
            PF1<Alternate<AF5>>
        ]
        MISO: [
            PA10<Alternate<AF5>>
        ]
        MOSI: [
            PA11<Alternate<AF5>>
        ]
}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398",
))]
pins! {
    SPI2:
        SCK: [
            PF9<Alternate<AF5>>,
            PF10<Alternate<AF5>>
        ]
        MISO: []
        MOSI: []
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107"
))]
pins! {
    SPI3:
        SCK: [
            NoSck,
            PB3<Alternate<PushPull>>
        ]
        MISO: [
            NoMiso,
            PB4<Input<Floating>>
        ]
        MOSI: [
            NoMosi,
            PB5<Alternate<PushPull>>
        ]
}

#[cfg(any(feature = "stm32f105", feature = "stm32f107"))]
pins! {
    SPI3:
        SCK: [
            PC10<Alternate<PushPull>>
        ]
        MISO: [
            PC11<Input<Floating>>
        ]
        MOSI: [
            PC12<Alternate<PushPull>>
        ]
}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI3:
        SCK: [
            NoSck,
            PB3<Alternate<AF6>>,
            PC10<Alternate<AF6>>
        ]
        MISO: [
            NoMiso,
            PB4<Alternate<AF6>>,
            PC11<Alternate<AF6>>
        ]
        MOSI: [
            NoMosi,
            PB5<Alternate<AF6>>,
            PC12<Alternate<AF6>>
        ]
}

#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
pins! {
    SPI3:
        SCK: [
            PA1<Alternate<AF6>>
        ]
        MISO: [
            PA2<Alternate<AF6>>
        ]
        MOSI: [
            PA3<Alternate<AF6>>
        ]
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI2:
        SCK: [PD3<Alternate<AF5>>]
        MISO: []
        MOSI: []
    SPI3:
        SCK: []
        MISO: []
        MOSI: [PD6<Alternate<AF5>>]
}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI4:
        SCK: [
            NoSck,
            PE2<Alternate<AF5>>,
            PE12<Alternate<AF5>>
        ]
        MISO: [
            NoMiso,
            PE5<Alternate<AF5>>,
            PE13<Alternate<AF5>>
        ]
        MOSI: [
            NoMosi,
            PE6<Alternate<AF5>>,
            PE14<Alternate<AF5>>
        ]
}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI2:
        SCK: [PI1<Alternate<AF5>>]
        MISO: [PI2<Alternate<AF5>>]
        MOSI: [PI3<Alternate<AF5>>]
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
pins! {
    SPI2:
        SCK: [PC7<Alternate<AF5>>]
        MISO: []
        MOSI: []
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pins! {
    SPI5:
        SCK: [
            NoSck,
            PB0<Alternate<AF6>>
        ]
        MISO: [
            NoMiso,
            PA12<Alternate<AF6>>
        ]
        MOSI: [
            NoMosi,
            PA10<Alternate<AF6>>,
            PB8<Alternate<AF6>>
        ]
}

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pins! {
    SPI3:
        SCK: [PB12<Alternate<AF7>>]
        MISO: []
        MOSI: []
    SPI4:
        SCK: [PB13<Alternate<AF6>>]
        MISO: [PA11<Alternate<AF6>>]
        MOSI: [PA1<Alternate<AF5>>]
    SPI5:
        SCK: [
            PE2<Alternate<AF6>>,
            PE12<Alternate<AF6>>
        ]
        MISO: [
            PE5<Alternate<AF6>>,
            PE13<Alternate<AF6>>
        ]
        MOSI: [
            PE6<Alternate<AF6>>,
            PE14<Alternate<AF6>>
        ]
}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pins! {
    SPI2:
        SCK: [PA9<Alternate<AF5>>]
        MISO: [PA12<Alternate<AF5>>]
        MOSI: [PA10<Alternate<AF5>>]
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI5:
        SCK: [
            NoSck,
            PF7<Alternate<AF5>>,
            PH6<Alternate<AF5>>
        ]
        MISO: [
            NoMiso,
            PF8<Alternate<AF5>>,
            PH7<Alternate<AF5>>
        ]
        MOSI: [
            NoMosi,
            PF9<Alternate<AF5>>,
            PF11<Alternate<AF5>>
        ]

    SPI6:
        SCK: [
            NoSck,
            PG13<Alternate<AF5>>
        ]
        MISO: [
            NoMiso,
            PG12<Alternate<AF5>>
        ]
        MOSI: [
            NoMosi,
            PG14<Alternate<AF5>>
        ]
}

#[cfg(any(feature = "stm32f446"))]
pins! {
    SPI2:
        SCK: [PA9<Alternate<AF5>>]
        MISO: []
        MOSI: [PC1<Alternate<AF7>>]

    SPI3:
        SCK: []
        MISO: []
        MOSI: [
            PB0<Alternate<AF7>>,
            PB2<Alternate<AF7>>,
            PD0<Alternate<AF6>>
        ]

    SPI4:
        SCK: [PG11<Alternate<AF6>>]
        MISO: [
            PG12<Alternate<AF6>>,
            PD0<Alternate<AF5>>
        ]
        MOSI: [PG13<Alternate<AF6>>]
}

#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
pins! {
    SPI2:
        SCK: [PA9<Alternate<AF5>>]
        MISO: []
        MOSI: [PC1<Alternate<AF5>>]
}

/// Interrupt events
pub enum Event {
    /// New data has been received
    Rxne,
    /// Data can be sent
    Txe,
    /// An error occurred
    Error,
}

#[derive(Debug)]
pub struct Spi<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f328",
    feature = "stm32f334",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl<PINS> Spi<SPI1, PINS> {
    pub fn spi1(spi: SPI1, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
    where
        PINS: Pins<SPI1>
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for SPI
        rcc.apb2enr.modify(|_, w| w.spi1en().set_bit());

        Spi { spi, pins }.init(mode, freq, clocks.pclk2())
    }
}

#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107",
    feature = "stm32f2",
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl<PINS> Spi<SPI2, PINS> {
    pub fn spi2(spi: SPI2, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
    where
        PINS: Pins<SPI2>
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for SPI
        rcc.apb1enr.modify(|_, w| w.spi2en().set_bit());

        Spi { spi, pins }.init(mode, freq, clocks.pclk1())
    }
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107",
    feature = "stm32f2",
    feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f318",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl<PINS> Spi<SPI3, PINS> {
    pub fn spi3(spi: SPI3, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
    where
        PINS: Pins<SPI3>
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for SPI
        rcc.apb1enr.modify(|_, w| w.spi3en().set_bit());

        Spi { spi, pins }.init(mode, freq, clocks.pclk1())
    }
}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f398",
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl<PINS> Spi<SPI4, PINS> {
    pub fn spi4(spi: SPI4, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
    where
        PINS: Pins<SPI4>
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for SPI
        rcc.apb2enr.modify(|_, w| w.spi4en().set_bit());

        Spi { spi, pins }.init(mode, freq, clocks.pclk2())
    }
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl<PINS> Spi<SPI5, PINS> {
    pub fn spi5(spi: SPI5, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
    where
        PINS: Pins<SPI5>
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for SPI
        rcc.apb2enr.modify(|_, w| w.spi5en().set_bit());

        Spi { spi, pins }.init(mode, freq, clocks.pclk2())
    }
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl<PINS> Spi<SPI6, PINS> {
    pub fn spi6(spi: SPI6, pins: PINS, mode: Mode, freq: Hertz, clocks: Clocks) -> Self
    where
        PINS: Pins<SPI6>
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for SPI
        rcc.apb2enr.modify(|_, w| w.spi6en().set_bit());

        Spi { spi, pins }.init(mode, freq, clocks.pclk2())
    }
}

impl<SPI, PINS> Spi<SPI, PINS>
where
    SPI: Deref<Target = spi1::RegisterBlock>,
{
    pub fn init(self, mode: Mode, freq: Hertz, clock: Hertz) -> Self
    {
        // disable SS output
        // ds: 8 bit frames
        self.spi.cr2.write(|w| {
            #[cfg(feature = "stm32f0")]
            let w = w.ds().eight_bit();
            w.ssoe().clear_bit()
        });

        let br = match clock.0 / freq.0 {
            0 => unreachable!(),
            1...2 => 0b000,
            3...5 => 0b001,
            6...11 => 0b010,
            12...23 => 0b011,
            24...47 => 0b100,
            48...95 => 0b101,
            96...191 => 0b110,
            _ => 0b111,
        };

        // mstr: master configuration
        // lsbfirst: MSB first
        // ssm: enable software slave management (NSS pin free for other uses)
        // ssi: set nss high = master mode
        // dff: 8 bit frames
        // bidimode: 2-line unidirectional
        // spe: enable the SPI bus
        self.spi.cr1.write(|w| {
            #[cfg(any(
                feature = "stm32f1",
                feature = "stm32f2",
                feature = "stm32f4"
            ))]
            let w = w.dff().clear_bit();
            w.cpha()
            .bit(mode.phase == Phase::CaptureOnSecondTransition)
            .cpol()
            .bit(mode.polarity == Polarity::IdleHigh)
            .mstr()
            .set_bit()
            .br()
            .bits(br)
            .lsbfirst()
            .clear_bit()
            .ssm()
            .set_bit()
            .ssi()
            .set_bit()
            .rxonly()
            .clear_bit()
            .bidimode()
            .clear_bit()
            .spe()
            .set_bit()
        });

        self
    }

    /// Enable interrupts for the given `event`:
    ///  - Received data ready to be read (RXNE)
    ///  - Transmit data register empty (TXE)
    ///  - Transfer error
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::Rxne  => self.spi.cr2.modify(|_, w| { w.rxneie().set_bit() }),
            Event::Txe   => self.spi.cr2.modify(|_, w| { w.txeie().set_bit() }),
            Event::Error => self.spi.cr2.modify(|_, w| { w.errie().set_bit() }),
        }
    }

    /// Disable interrupts for the given `event`:
    ///  - Received data ready to be read (RXNE)
    ///  - Transmit data register empty (TXE)
    ///  - Transfer error
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::Rxne  => self.spi.cr2.modify(|_, w| { w.rxneie().clear_bit() }),
            Event::Txe   => self.spi.cr2.modify(|_, w| { w.txeie().clear_bit() }),
            Event::Error => self.spi.cr2.modify(|_, w| { w.errie().clear_bit() }),
        }
    }

    /// Return `true` if the TXE flag is set, i.e. new data to transmit
    /// can be written to the SPI.
    pub fn is_txe(&self) -> bool {
        self.spi.sr.read().txe().bit_is_set()
    }

    /// Return `true` if the RXNE flag is set, i.e. new data has been received
    /// and can be read from the SPI.
    pub fn is_rxne(&self) -> bool {
        self.spi.sr.read().rxne().bit_is_set()
    }

    /// Return `true` if the MODF flag is set, i.e. the SPI has experienced a
    /// Master Mode Fault. (see chapter 28.3.10 of the STM32F4 Reference Manual)
    pub fn is_modf(&self) -> bool {
        self.spi.sr.read().modf().bit_is_set()
    }

    /// Return `true` if the OVR flag is set, i.e. new data has been received
    /// while the receive data register was already filled.
    pub fn is_ovr(&self) -> bool {
        self.spi.sr.read().ovr().bit_is_set()
    }

    pub fn free(self) -> (SPI, PINS) {
        (self.spi, self.pins)
    }
}

impl<SPI, PINS> spi::FullDuplex<u8> for Spi<SPI, PINS>
where
    SPI: Deref<Target = spi1::RegisterBlock>,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        let sr = self.spi.sr.read();

        Err(if sr.ovr().bit_is_set() {
            nb::Error::Other(Error::Overrun)
        } else if sr.modf().bit_is_set() {
            nb::Error::Other(Error::ModeFault)
        } else if sr.crcerr().bit_is_set() {
            nb::Error::Other(Error::Crc)
        } else if sr.rxne().bit_is_set() {
            // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows
            // reading a half-word)
            return Ok(unsafe {
                ptr::read_volatile(&self.spi.dr as *const _ as *const u8)
            });
        } else {
            nb::Error::WouldBlock
        })
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
        let sr = self.spi.sr.read();

        Err(if sr.ovr().bit_is_set() {
            nb::Error::Other(Error::Overrun)
        } else if sr.modf().bit_is_set() {
            nb::Error::Other(Error::ModeFault)
        } else if sr.crcerr().bit_is_set() {
            nb::Error::Other(Error::Crc)
        } else if sr.txe().bit_is_set() {
            // NOTE(write_volatile) see note above
            unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut u8, byte) }
            return Ok(());
        } else {
            nb::Error::WouldBlock
        })
    }
}

impl<SPI, PINS> embedded_hal::blocking::spi::transfer::Default<u8> for Spi<SPI, PINS>
where
    SPI: Deref<Target = spi1::RegisterBlock>,
{}

impl<SPI, PINS> embedded_hal::blocking::spi::write::Default<u8> for Spi<SPI, PINS>
where
    SPI: Deref<Target = spi1::RegisterBlock>,
{}
