//! # Quadrature Encoder Interface
use crate::hal::{self, Direction};
use crate::stm32::RCC;
#[cfg(feature = "stm32f1")]
use crate::stm32::AFIO;

use crate::gpio::gpioa::*;
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
use crate::gpio::gpiob::*;
#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f1",
    feature = "stm32f2",
    // feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f334",
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
use crate::gpio::gpioc::*;
#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107",
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
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
use crate::gpio::gpiod::*;
#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8",
    feature = "stm32f100",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107",
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
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
use crate::gpio::gpioe::*;
#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::gpiof::*;
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
use crate::gpio::gpioh::*;
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
use crate::gpio::gpioi::*;

#[cfg(feature = "stm32f1")]
use crate::gpio::{Floating, Input};

#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f2",
    feature = "stm32f3",
    feature = "stm32f4"
))]
use crate::gpio::Alternate;

#[cfg(feature = "stm32f0")]
use crate::gpio::AF0;

#[cfg(any(
    feature = "stm32f0",
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
use crate::gpio::AF1;

#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f2",
    // feature = "stm32f301",
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
use crate::gpio::AF2;

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f405",
    feature = "stm32f407",
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
use crate::gpio::AF3;

#[cfg(any(
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
use crate::gpio::AF4;

#[cfg(any(
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
use crate::gpio::AF5;

#[cfg(any(
    // feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    // feature = "stm32f318",
    feature = "stm32f328",
    feature = "stm32f334",
    feature = "stm32f358",
    feature = "stm32f398"
))]
use crate::gpio::AF6;

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398"
))]
use crate::gpio::AF10;

#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f100",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107",
    feature = "stm32f2",
    // feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    // feature = "stm32f318",
    feature = "stm32f328",
    feature = "stm32f334",
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
use crate::stm32::TIM1;

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8",
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f3",
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
use crate::stm32::TIM2;

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
use crate::stm32::TIM3;

#[cfg(any(
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
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
use crate::stm32::TIM4;

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107",
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
use crate::stm32::TIM5;

#[cfg(any(
    feature = "stm32f103",
    feature = "stm32f2",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398",
    feature = "stm32f405",
    feature = "stm32f407",
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
use crate::stm32::TIM8;

pub trait Pins<TIM> {
#[cfg(feature = "stm32f1")]
    fn remap() {}
}
pub trait PinC1<TIM> {}
pub trait PinC2<TIM> {}

impl<TIM, PC1, PC2> Pins<TIM> for (PC1, PC2)
where
    PC1: PinC1<TIM>,
    PC2: PinC2<TIM>,
{
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107"
))]
impl Pins<TIM1> for (PA8<Input<Floating>>, PA9<Input<Floating>>) {
    fn remap() {
        let afio = unsafe { &(*AFIO::ptr()) };
        afio.mapr.modify(|_, w| unsafe { w.tim1_remap().bits(0b00) });
    }
}
#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107"
))]
impl Pins<TIM1> for (PE9<Input<Floating>>, PE11<Input<Floating>>) {
    fn remap() {
        let afio = unsafe { &(*AFIO::ptr()) };
        afio.mapr.modify(|_, w| unsafe { w.tim1_remap().bits(0b11) });
    }
}

#[cfg(feature = "stm32f1")]
impl Pins<TIM2> for (PA0<Input<Floating>>, PA1<Input<Floating>>) {
    fn remap() {
        let afio = unsafe { &(*AFIO::ptr()) };
        afio.mapr.modify(|_, w| unsafe { w.tim2_remap().bits(0b00) });
    }
}
#[cfg(feature = "stm32f1")]
impl Pins<TIM2> for (PA15<Input<Floating>>, PB3<Input<Floating>>) {
    fn remap() {
        let afio = unsafe { &(*AFIO::ptr()) };
        afio.mapr.modify(|_, w| unsafe { w.tim2_remap().bits(0b11) });
    }
}

#[cfg(feature = "stm32f1")]
impl Pins<TIM3> for (PA6<Input<Floating>>, PA7<Input<Floating>>) {
    fn remap() {
        let afio = unsafe { &(*AFIO::ptr()) };
        afio.mapr.modify(|_, w| unsafe { w.tim3_remap().bits(0b00) });
    }
}
#[cfg(feature = "stm32f1")]
impl Pins<TIM3> for (PC6<Input<Floating>>, PC7<Input<Floating>>) {
    fn remap() {
        let afio = unsafe { &(*AFIO::ptr()) };
        afio.mapr.modify(|_, w| unsafe { w.tim3_remap().bits(0b11) });
    }
}

#[cfg(feature = "stm32f1")]
impl Pins<TIM4> for (PB6<Input<Floating>>, PB7<Input<Floating>>) {
    fn remap() {
        let afio = unsafe { &(*AFIO::ptr()) };
        afio.mapr.modify(|_, w| w.tim4_remap().clear_bit() );
    }
}
#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107"
))]
impl Pins<TIM4> for (PD12<Input<Floating>>, PD13<Input<Floating>>) {
    fn remap() {
        let afio = unsafe { &(*AFIO::ptr()) };
        afio.mapr.modify(|_, w| w.tim4_remap().set_bit() );
    }
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107"
))]
impl Pins<TIM5> for (PA0<Input<Floating>>, PA1<Input<Floating>>) {}

#[cfg(feature = "stm32f103")]
impl Pins<TIM8> for (PC6<Input<Floating>>, PC7<Input<Floating>>) {}

#[cfg(feature = "stm32f0")]
impl PinC1<TIM1> for PA8<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
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
impl PinC1<TIM1> for PA8<Alternate<AF1>> {}

#[cfg(any(
    // feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    // feature = "stm32f318",
    feature = "stm32f328",
    feature = "stm32f334",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC1<TIM1> for PA8<Alternate<AF6>> {}

#[cfg(feature = "stm32f0")]
impl PinC2<TIM1> for PA9<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
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
impl PinC2<TIM1> for PA9<Alternate<AF1>> {}

#[cfg(any(
    // feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    // feature = "stm32f318",
    feature = "stm32f328",
    feature = "stm32f334",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC2<TIM1> for PA9<Alternate<AF6>> {}

#[cfg(any(
    // feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f334",
    feature = "stm32f398"
))]
impl PinC1<TIM1> for PC0<Alternate<AF2>> {}

#[cfg(any(
    // feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f334",
    feature = "stm32f398"
))]
impl PinC2<TIM1> for PC1<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
impl PinC1<TIM1> for PE9<Alternate<AF0>> {}

#[cfg(any(
    feature = "stm32f2",
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
impl PinC1<TIM1> for PE9<Alternate<AF1>> {}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC1<TIM1> for PE9<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
impl PinC2<TIM1> for PE11<Alternate<AF0>> {}

#[cfg(any(
    feature = "stm32f2",
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
impl PinC2<TIM1> for PE11<Alternate<AF1>> {}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC2<TIM1> for PE11<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
impl PinC1<TIM2> for PA0<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f3",
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
impl PinC1<TIM2> for PA0<Alternate<AF1>> {}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
impl PinC1<TIM2> for PA5<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f3",
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
impl PinC1<TIM2> for PA5<Alternate<AF1>> {}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
impl PinC1<TIM2> for PA15<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f3",
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
impl PinC1<TIM2> for PA15<Alternate<AF1>> {}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
impl PinC2<TIM2> for PA1<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f3",
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
impl PinC2<TIM2> for PA1<Alternate<AF1>> {}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
impl PinC2<TIM2> for PB3<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f3",
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
impl PinC2<TIM2> for PB3<Alternate<AF1>> {}

#[cfg(feature = "stm32f446")]
impl PinC1<TIM2> for PB8<Alternate<AF1>> {}

#[cfg(feature = "stm32f446")]
impl PinC2<TIM2> for PB9<Alternate<AF1>> {}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC1<TIM2> for PD3<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC2<TIM2> for PD4<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f328",
    feature = "stm32f334",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398"
))]
impl PinC2<TIM3> for PA4<Alternate<AF2>> {}

#[cfg(feature = "stm32f0")]
impl PinC1<TIM3> for PA6<Alternate<AF1>> {}

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
impl PinC1<TIM3> for PA6<Alternate<AF2>> {}

#[cfg(feature = "stm32f0")]
impl PinC2<TIM3> for PA7<Alternate<AF1>> {}

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
impl PinC2<TIM3> for PA7<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
impl PinC1<TIM3> for PB0<Alternate<AF10>> {}

#[cfg(feature = "stm32f0")]
impl PinC1<TIM3> for PB4<Alternate<AF1>> {}

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
impl PinC1<TIM3> for PB4<Alternate<AF2>> {}

#[cfg(feature = "stm32f0")]
impl PinC2<TIM3> for PB5<Alternate<AF1>> {}

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
impl PinC2<TIM3> for PB5<Alternate<AF2>> {}

#[cfg(feature = "stm32f0")]
impl PinC1<TIM3> for PC6<Alternate<AF0>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f334",
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
impl PinC1<TIM3> for PC6<Alternate<AF2>> {}

#[cfg(feature = "stm32f0")]
impl PinC2<TIM3> for PC7<Alternate<AF0>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f334",
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
impl PinC2<TIM3> for PC7<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC1<TIM3> for PE2<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC2<TIM3> for PE3<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
impl PinC1<TIM3> for PE3<Alternate<AF0>> {}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8"
))]
impl PinC2<TIM3> for PE4<Alternate<AF0>> {}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398"
))]
impl PinC1<TIM3> for PA11<Alternate<AF10>> {}

#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f398"
))]
impl PinC2<TIM3> for PA12<Alternate<AF10>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
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
impl PinC1<TIM4> for PB6<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
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
impl PinC2<TIM4> for PB7<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
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
impl PinC1<TIM4> for PD12<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
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
impl PinC2<TIM4> for PD13<Alternate<AF2>> {}

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
impl PinC1<TIM5> for PA0<Alternate<AF2>> {}

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
impl PinC2<TIM5> for PA1<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
impl PinC1<TIM5> for PA8<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
impl PinC2<TIM5> for PA11<Alternate<AF2>> {}

#[cfg(feature = "stm32f410")]
impl PinC1<TIM5> for PB12<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
impl PinC1<TIM5> for PC0<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f373",
    feature = "stm32f378"
))]
impl PinC2<TIM5> for PC1<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinC1<TIM5> for PF3<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinC2<TIM5> for PF4<Alternate<AF2>> {}

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
impl PinC1<TIM5> for PH10<Alternate<AF2>> {}

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
impl PinC2<TIM5> for PH11<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC2<TIM8> for PA14<Alternate<AF5>> {}

#[cfg(any(
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC1<TIM8> for PA15<Alternate<AF2>> {}

#[cfg(any(
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC1<TIM8> for PB6<Alternate<AF5>> {}

#[cfg(any(
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC2<TIM8> for PB8<Alternate<AF10>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f405",
    feature = "stm32f407",
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
impl PinC1<TIM8> for PC6<Alternate<AF3>> {}

#[cfg(any(
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC1<TIM8> for PC6<Alternate<AF4>> {}

#[cfg(any(
    feature = "stm32f2",
    feature = "stm32f405",
    feature = "stm32f407",
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
impl PinC2<TIM8> for PC7<Alternate<AF3>> {}

#[cfg(any(
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398"
))]
impl PinC1<TIM8> for PC7<Alternate<AF4>> {}

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
impl PinC1<TIM8> for PI5<Alternate<AF3>> {}

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
impl PinC2<TIM8> for PI6<Alternate<AF3>> {}

/// Hardware quadrature encoder interface peripheral
pub struct Qei<TIM, PINS> {
    tim: TIM,
    pins: PINS,
}

macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $timXen:ident, $timXrst:ident, $apbenr:ident, $apbrstr:ident, $bits:ident),)+) => {
        $(
            impl<PINS> Qei<$TIM, PINS> {
                /// Configures a TIM peripheral as a quadrature encoder interface input
                pub fn $tim(tim: $TIM, pins: PINS) -> Self
                where
                    PINS: Pins<$TIM>
                {
                    let rcc = unsafe { &(*RCC::ptr()) };
                    // enable and reset peripheral to a clean slate state
                    rcc.$apbenr.modify(|_, w| w.$timXen().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().clear_bit());

                    #[cfg(feature = "stm32f1")]
                    PINS::remap();

                    // Configure TxC1 and TxC2 as captures
                    tim.ccmr1_output
                        .write(|w| unsafe { w.cc1s().bits(0b01).cc2s().bits(0b01) });

                    // enable and configure to capture on rising edge
                    tim.ccer.write(|w| {
                        w.cc1e()
                            .set_bit()
                            .cc1p()
                            .clear_bit()
                            .cc2e()
                            .set_bit()
                            .cc2p()
                            .clear_bit()
                    });

                    // configure as quadrature encoder
                    // some chip variants declare `.bits()` as unsafe, some don't
                    #[allow(unused_unsafe)]
                    tim.smcr.write(|w| unsafe { w.sms().bits(3) });

                    tim.arr.write(|w| unsafe { w.bits(core::u32::MAX) });
                    tim.cr1.write(|w| w.cen().set_bit());

                    Qei { tim, pins }
                }

                /// Releases the TIM peripheral and QEI pins
                pub fn release(self) -> ($TIM, PINS) {
                    (self.tim, self.pins)
                }
            }

            impl<PINS> hal::Qei for Qei<$TIM, PINS> {
                type Count = $bits;

                fn count(&self) -> $bits {
                    self.tim.cnt.read().bits() as $bits
                }

                fn direction(&self) -> Direction {
                    if self.tim.cr1.read().dir().bit_is_clear() {
                        hal::Direction::Upcounting
                    } else {
                        hal::Direction::Downcounting
                    }
                }
            }

        )+
    }
}

#[cfg(any(
    feature = "stm32f0",
    feature = "stm32f100",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107",
    feature = "stm32f2",
    // feature = "stm32f301",
    feature = "stm32f302",
    feature = "stm32f303",
    // feature = "stm32f318",
    feature = "stm32f328",
    feature = "stm32f334",
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
hal! {
    TIM1: (tim1, tim1en, tim1rst, apb2enr, apb2rstr, u16),
}

#[cfg(any(
    feature = "stm32f0x1",
    feature = "stm32f0x2",
    feature = "stm32f0x8",
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f3",
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
hal! {
    TIM2: (tim2, tim2en, tim2rst, apb1enr, apb1rstr, u32),
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
hal! {
    TIM3: (tim3, tim3en, tim3rst, apb1enr, apb1rstr, u16),
}

#[cfg(any(
    feature = "stm32f1",
    feature = "stm32f2",
    feature = "stm32f302",
    feature = "stm32f303",
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
hal! {
    TIM4: (tim4, tim4en, tim4rst, apb1enr, apb1rstr, u16),
}

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107",
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
hal! {
    TIM5: (tim5, tim5en, tim5rst, apb1enr, apb1rstr, u32),
}

#[cfg(any(
    feature = "stm32f103",
    feature = "stm32f2",
    feature = "stm32f303",
    feature = "stm32f358",
    feature = "stm32f398",
    feature = "stm32f405",
    feature = "stm32f407",
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
hal! {
    TIM8: (tim8, tim8en, tim8rst, apb2enr, apb2rstr, u16),
}
