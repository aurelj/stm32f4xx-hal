//! General Purpose Input / Output

use core::marker::PhantomData;

use crate::stm32::EXTI;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

/// Analog mode (type state)
pub struct Analog;

/// Alternate function
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// GPIO Pin speed selection
pub enum Speed {
    Low = 0,
    Medium = 1,
    High = 2,
    VeryHigh = 3,
}

#[derive(Debug, PartialEq)]
pub enum Edge {
    RISING,
    FALLING,
    RISING_FALLING,
}

/// External Interrupt Pin
pub trait ExtiPin {
    fn make_interrupt_source(&mut self);
    fn trigger_on_edge(&mut self, exti: &mut EXTI, level: Edge);
    fn enable_interrupt(&mut self, exti: &mut EXTI);
    fn disable_interrupt(&mut self, exti: &mut EXTI);
    fn clear_interrupt_pending_bit(&mut self, exti: &mut EXTI);
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $iopxenr:ident, $PXx:ident, $extigpionr:expr, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty, $exticri:ident),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;

            use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, toggleable};
            use crate::stm32::$GPIOX;

            use crate::stm32::{RCC, EXTI, AFIO};
            use super::{
                Alternate, Floating, GpioExt, Input,
                OpenDrain,
                Output,
                PullDown,
                PullUp,
                PushPull,
                Analog,
                Speed,
                Edge,
                ExtiPin,
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    let rcc = unsafe { &(*RCC::ptr()) };
                    rcc.apb2enr.modify(|_, w| w.$iopxenr().set_bit());

                    Parts {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Partially erased pin
            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> $PXx<MODE> {
                pub fn get_id(&self) -> u8 {
                    self.i
                }
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                fn set_high(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << self.i)) }
                }

                fn set_low(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + self.i))) }
                }
            }

            impl<MODE> InputPin for $PXx<Input<MODE>> {
                fn is_high(&self) -> bool {
                    !self.is_low()
                }

                fn is_low(&self) -> bool {
                    // NOTE(unsafe) atomic read with no side effects
                    unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << self.i) == 0 }
                }
            }

            impl<MODE> StatefulOutputPin for $PXx<Output<MODE>> {
                fn is_set_high(&self) -> bool {
                    !self.is_set_low()
                }

                fn is_set_low(&self) -> bool {
                    // NOTE(unsafe) atomic read with no side effects
                    unsafe { (*$GPIOX::ptr()).odr.read().bits() & (1 << self.i) == 0 }
                }
            }

            impl<MODE> toggleable::Default for $PXx<Output<MODE>> {}

            impl<MODE> InputPin for $PXx<Output<MODE>> {
                fn is_high(&self) -> bool {
                    !self.is_low()
                }

                fn is_low(&self) -> bool {
                    // NOTE(unsafe) atomic read with no side effects
                    unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << self.i) == 0 }
                }
            }

            impl<MODE> ExtiPin for $PXx<Input<MODE>> {
                /// Make corresponding EXTI line sensitive to this pin
                fn make_interrupt_source(&mut self) {
                    let afio = unsafe { &(*AFIO::ptr()) };
                    let offset = 4 * (self.i % 4);
                    match self.i {
                        0...3 => {
                            afio.exticr1.modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0xf << offset)) | ($extigpionr << offset))
                            });
                        },
                        4...7 => {
                            afio.exticr2.modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0xf << offset)) | ($extigpionr << offset))
                            });
                        },
                        8...11 => {
                            afio.exticr3.modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0xf << offset)) | ($extigpionr << offset))
                            });
                        },
                        12...15 => {
                            afio.exticr4.modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0xf << offset)) | ($extigpionr << offset))
                            });
                        },
                        _ => {}
                    }
                }

                /// Generate interrupt on rising edge, falling edge or both
                fn trigger_on_edge(&mut self, exti: &mut EXTI, edge: Edge) {
                    match edge {
                        Edge::RISING => {
                            exti.rtsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.i)) });
                            exti.ftsr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.i)) });
                        },
                        Edge::FALLING => {
                            exti.ftsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.i)) });
                            exti.rtsr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.i)) });
                        },
                        Edge::RISING_FALLING => {
                            exti.rtsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.i)) });
                            exti.ftsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.i)) });
                        }
                    }
                }

                /// Enable external interrupts from this pin.
                fn enable_interrupt(&mut self, exti: &mut EXTI) {
                    exti.imr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.i)) });
                }

                /// Disable external interrupts from this pin
                fn disable_interrupt(&mut self, exti: &mut EXTI) {
                    exti.imr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.i)) });
                }

                /// Clear the interrupt pending bit for this pin
                fn clear_interrupt_pending_bit(&mut self, exti: &mut EXTI) {
                    exti.pr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.i)) });
                }
            }

            fn set_configuration(index: usize, cnf: u32, mode: u32) {
                let offset = 4 * index;
                let bits = (cnf << 2) | mode;

                unsafe {
                    if offset < 32 {
                        (*$GPIOX::ptr()).crl.modify(|r, w|
                            w.bits((r.bits() & !(0b1111 << offset)) | (bits << offset))
                        );
                    } else {
                        let offset = offset - 32;
                        (*$GPIOX::ptr()).crh.modify(|r, w|
                            w.bits((r.bits() & !(0b1111 << offset)) | (bits << offset))
                        );
                    }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate as an alternate function push pull output pin
                    pub fn into_alternate(self) -> $PXi<Alternate<PushPull>> {
                        // Alternate function output push pull / Output mode, 50 MHz
                        set_configuration($i, 0b10, 0b11);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(self) -> $PXi<Input<Floating>> {
                        // Floating input / Input mode
                        set_configuration($i, 0b01, 0b00);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(self) -> $PXi<Input<PullDown>> {
                        //pull down:
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + $i))) };

                        // Pull up/down input / Input mode
                        set_configuration($i, 0b10, 0b00);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(self) -> $PXi<Input<PullUp>> {
                        //pull up:
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) };

                        // Pull up/down input / Input mode
                        set_configuration($i, 0b10, 0b00);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(self) -> $PXi<Output<OpenDrain>> {
                        // General purpose output open-drain / Output mode, 50 MHz
                        set_configuration($i, 0b01, 0b11);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(self) -> $PXi<Output<PushPull>> {
                        // General purpose output push-pull / Output mode, 50 MHz
                        set_configuration($i, 0b00, 0b11);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an analog input pin
                    pub fn into_analog(self) -> $PXi<Analog> {
                        // Analog input / Input mode
                        set_configuration($i, 0b00, 0b00);
                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    /// Set pin speed
                    pub fn set_speed(self, speed: Speed) -> Self {
                        let offset = 4 * $i;
                        let bits = match speed {
                            Speed::Low => 0b10,
                            Speed::Medium => 0b01,
                            Speed::High => 0b11,
                            Speed::VeryHigh => 0b11,
                        };

                        unsafe {
                            if offset < 32 {
                                (*$GPIOX::ptr()).crl.modify(|r, w|
                                    w.bits((r.bits() & !(0b11 << offset)) | (bits << offset))
                                );
                            } else {
                                let offset = offset - 32;
                                (*$GPIOX::ptr()).crh.modify(|r, w|
                                    w.bits((r.bits() & !(0b11 << offset)) | (bits << offset))
                                );
                            }
                        }

                        self
                    }
                }

                impl<MODE> $PXi<Alternate<MODE>> {
                    /// Set pin speed
                    pub fn set_speed(self, speed: Speed) -> Self {
                        let offset = 4 * $i;
                        let bits = match speed {
                            Speed::Low => 0b10,
                            Speed::Medium => 0b01,
                            Speed::High => 0b11,
                            Speed::VeryHigh => 0b11,
                        };

                        unsafe {
                            if offset < 32 {
                                (*$GPIOX::ptr()).crl.modify(|r, w|
                                    w.bits((r.bits() & !(0b11 << offset)) | (bits << offset))
                                );
                            } else {
                                let offset = offset - 32;
                                (*$GPIOX::ptr()).crh.modify(|r, w|
                                    w.bits((r.bits() & !(0b11 << offset)) | (bits << offset))
                                );
                            }
                        }

                        self
                    }

                    /// Turns pin alternate configuration pin into open drain
                    pub fn set_open_drain(self) -> $PXi<Alternate<OpenDrain>> {
                        let offset = 4 * $i + 2;

                        unsafe {
                            if offset < 32 {
                                (*$GPIOX::ptr()).crl.modify(|r, w|
                                    w.bits((r.bits() & !(0b11 << offset)) | (0b11 << offset))
                                );
                            } else {
                                let offset = offset - 32;
                                (*$GPIOX::ptr()).crh.modify(|r, w|
                                    w.bits((r.bits() & !(0b11 << offset)) | (0b11 << offset))
                                );
                            }
                        }

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> $PXi<MODE> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<MODE> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set_high(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + $i))) }
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
                    fn is_set_high(&self) -> bool {
                        !self.is_set_low()
                    }

                    fn is_set_low(&self) -> bool {
                        // NOTE(unsafe) atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).odr.read().bits() & (1 << $i) == 0 }
                    }
                }

                impl<MODE> toggleable::Default for $PXi<Output<MODE>> {}

                impl<MODE> InputPin for $PXi<Output<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        // NOTE(unsafe) atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << $i) == 0 }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        // NOTE(unsafe) atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << $i) == 0 }
                    }
                }

                impl<MODE> ExtiPin for $PXi<Input<MODE>> {
                    /// Configure EXTI Line $i to trigger from this pin.
                    fn make_interrupt_source(&mut self) {
                        let afio = unsafe { &(*AFIO::ptr()) };
                        let offset = 4 * ($i % 4);
                        afio.$exticri.modify(|r, w| unsafe {
                            let mut exticr = r.bits();
                            exticr = (exticr & !(0xf << offset)) | ($extigpionr << offset);
                            w.bits(exticr)
                        });
                    }

                    /// Generate interrupt on rising edge, falling edge or both
                    fn trigger_on_edge(&mut self, exti: &mut EXTI, edge: Edge) {
                        match edge {
                            Edge::RISING => {
                                exti.rtsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << $i)) });
                                exti.ftsr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $i)) });
                            },
                            Edge::FALLING => {
                                exti.ftsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << $i)) });
                                exti.rtsr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $i)) });
                            },
                            Edge::RISING_FALLING => {
                                exti.rtsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << $i)) });
                                exti.ftsr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << $i)) });
                            }
                        }
                    }

                    /// Enable external interrupts from this pin.
                    fn enable_interrupt(&mut self, exti: &mut EXTI) {
                        exti.imr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << $i)) });
                    }

                    /// Disable external interrupts from this pin
                    fn disable_interrupt(&mut self, exti: &mut EXTI) {
                        exti.imr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $i)) });
                    }

                    /// Clear the interrupt pending bit for this pin
                    fn clear_interrupt_pending_bit(&mut self, exti: &mut EXTI) {
                        exti.pr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << $i)) });
                    }
                }
            )+
        }
    }
}

gpio!(GPIOA, gpioa, iopaen, PAx, 0, [
    PA0: (pa0, 0, Input<Floating>, exticr1),
    PA1: (pa1, 1, Input<Floating>, exticr1),
    PA2: (pa2, 2, Input<Floating>, exticr1),
    PA3: (pa3, 3, Input<Floating>, exticr1),
    PA4: (pa4, 4, Input<Floating>, exticr2),
    PA5: (pa5, 5, Input<Floating>, exticr2),
    PA6: (pa6, 6, Input<Floating>, exticr2),
    PA7: (pa7, 7, Input<Floating>, exticr2),
    PA8: (pa8, 8, Input<Floating>, exticr3),
    PA9: (pa9, 9, Input<Floating>, exticr3),
    PA10: (pa10, 10, Input<Floating>, exticr3),
    PA11: (pa11, 11, Input<Floating>, exticr3),
    PA12: (pa12, 12, Input<Floating>, exticr4),
    PA13: (pa13, 13, Input<Floating>, exticr4),
    PA14: (pa14, 14, Input<Floating>, exticr4),
    PA15: (pa15, 15, Input<Floating>, exticr4),
]);

gpio!(GPIOB, gpiob, iopben, PBx, 1, [
    PB0: (pb0, 0, Input<Floating>, exticr1),
    PB1: (pb1, 1, Input<Floating>, exticr1),
    PB2: (pb2, 2, Input<Floating>, exticr1),
    PB3: (pb3, 3, Input<Floating>, exticr1),
    PB4: (pb4, 4, Input<Floating>, exticr2),
    PB5: (pb5, 5, Input<Floating>, exticr2),
    PB6: (pb6, 6, Input<Floating>, exticr2),
    PB7: (pb7, 7, Input<Floating>, exticr2),
    PB8: (pb8, 8, Input<Floating>, exticr3),
    PB9: (pb9, 9, Input<Floating>, exticr3),
    PB10: (pb10, 10, Input<Floating>, exticr3),
    PB11: (pb11, 11, Input<Floating>, exticr3),
    PB12: (pb12, 12, Input<Floating>, exticr4),
    PB13: (pb13, 13, Input<Floating>, exticr4),
    PB14: (pb14, 14, Input<Floating>, exticr4),
    PB15: (pb15, 15, Input<Floating>, exticr4),
]);

gpio!(GPIOC, gpioc, iopcen, PCx, 2, [
    PC0: (pc0, 0, Input<Floating>, exticr1),
    PC1: (pc1, 1, Input<Floating>, exticr1),
    PC2: (pc2, 2, Input<Floating>, exticr1),
    PC3: (pc3, 3, Input<Floating>, exticr1),
    PC4: (pc4, 4, Input<Floating>, exticr2),
    PC5: (pc5, 5, Input<Floating>, exticr2),
    PC6: (pc6, 6, Input<Floating>, exticr2),
    PC7: (pc7, 7, Input<Floating>, exticr2),
    PC8: (pc8, 8, Input<Floating>, exticr3),
    PC9: (pc9, 9, Input<Floating>, exticr3),
    PC10: (pc10, 10, Input<Floating>, exticr3),
    PC11: (pc11, 11, Input<Floating>, exticr3),
    PC12: (pc12, 12, Input<Floating>, exticr4),
    PC13: (pc13, 13, Input<Floating>, exticr4),
    PC14: (pc14, 14, Input<Floating>, exticr4),
    PC15: (pc15, 15, Input<Floating>, exticr4),
]);

gpio!(GPIOD, gpiod, iopden, PDx, 3, [
    PD0: (pd0, 0, Input<Floating>, exticr1),
    PD1: (pd1, 1, Input<Floating>, exticr1),
    PD2: (pd2, 2, Input<Floating>, exticr1),
    PD3: (pd3, 3, Input<Floating>, exticr1),
    PD4: (pd4, 4, Input<Floating>, exticr2),
    PD5: (pd5, 5, Input<Floating>, exticr2),
    PD6: (pd6, 6, Input<Floating>, exticr2),
    PD7: (pd7, 7, Input<Floating>, exticr2),
    PD8: (pd8, 8, Input<Floating>, exticr3),
    PD9: (pd9, 9, Input<Floating>, exticr3),
    PD10: (pd10, 10, Input<Floating>, exticr3),
    PD11: (pd11, 11, Input<Floating>, exticr3),
    PD12: (pd12, 12, Input<Floating>, exticr4),
    PD13: (pd13, 13, Input<Floating>, exticr4),
    PD14: (pd14, 14, Input<Floating>, exticr4),
    PD15: (pd15, 15, Input<Floating>, exticr4),
]);

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103",
    feature = "stm32f105",
    feature = "stm32f107"
))]
gpio!(GPIOE, gpioe, iopeen, PEx, 4, [
    PE0: (pe0, 0, Input<Floating>, exticr1),
    PE1: (pe1, 1, Input<Floating>, exticr1),
    PE2: (pe2, 2, Input<Floating>, exticr1),
    PE3: (pe3, 3, Input<Floating>, exticr1),
    PE4: (pe4, 4, Input<Floating>, exticr2),
    PE5: (pe5, 5, Input<Floating>, exticr2),
    PE6: (pe6, 6, Input<Floating>, exticr2),
    PE7: (pe7, 7, Input<Floating>, exticr2),
    PE8: (pe8, 8, Input<Floating>, exticr3),
    PE9: (pe9, 9, Input<Floating>, exticr3),
    PE10: (pe10, 10, Input<Floating>, exticr3),
    PE11: (pe11, 11, Input<Floating>, exticr3),
    PE12: (pe12, 12, Input<Floating>, exticr4),
    PE13: (pe13, 13, Input<Floating>, exticr4),
    PE14: (pe14, 14, Input<Floating>, exticr4),
    PE15: (pe15, 15, Input<Floating>, exticr4),
]);

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103"
))]
gpio!(GPIOF, gpiof, iopfen, PFx, 5, [
    PF0: (pf0, 0, Input<Floating>, exticr1),
    PF1: (pf1, 1, Input<Floating>, exticr1),
    PF2: (pf2, 2, Input<Floating>, exticr1),
    PF3: (pf3, 3, Input<Floating>, exticr1),
    PF4: (pf4, 4, Input<Floating>, exticr2),
    PF5: (pf5, 5, Input<Floating>, exticr2),
    PF6: (pf6, 6, Input<Floating>, exticr2),
    PF7: (pf7, 7, Input<Floating>, exticr2),
    PF8: (pf8, 8, Input<Floating>, exticr3),
    PF9: (pf9, 9, Input<Floating>, exticr3),
    PF10: (pf10, 10, Input<Floating>, exticr3),
    PF11: (pf11, 11, Input<Floating>, exticr3),
    PF12: (pf12, 12, Input<Floating>, exticr3),
    PF13: (pf13, 13, Input<Floating>, exticr3),
    PF14: (pf14, 14, Input<Floating>, exticr3),
    PF15: (pf15, 15, Input<Floating>, exticr3),
]);

#[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f103"
))]
gpio!(GPIOG, gpiog, iopgen, PGx, 6, [
    PG0: (pg0, 0, Input<Floating>, exticr1),
    PG1: (pg1, 1, Input<Floating>, exticr1),
    PG2: (pg2, 2, Input<Floating>, exticr1),
    PG3: (pg3, 3, Input<Floating>, exticr1),
    PG4: (pg4, 4, Input<Floating>, exticr2),
    PG5: (pg5, 5, Input<Floating>, exticr2),
    PG6: (pg6, 6, Input<Floating>, exticr2),
    PG7: (pg7, 7, Input<Floating>, exticr2),
    PG8: (pg8, 8, Input<Floating>, exticr3),
    PG9: (pg9, 9, Input<Floating>, exticr3),
    PG10: (pg10, 10, Input<Floating>, exticr3),
    PG11: (pg11, 11, Input<Floating>, exticr3),
    PG12: (pg12, 12, Input<Floating>, exticr4),
    PG13: (pg13, 13, Input<Floating>, exticr4),
    PG14: (pg14, 14, Input<Floating>, exticr4),
    PG15: (pg15, 15, Input<Floating>, exticr4),
]);
