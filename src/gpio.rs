//! General Purpose Input / Output

use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

pub struct AF0;
pub struct AF1;
pub struct AF2;
pub struct AF3;
pub struct AF4;
pub struct AF5;
pub struct AF6;
pub struct AF7;
pub struct AF8;
pub struct AF9;
pub struct AF10;
pub struct AF11;
pub struct AF12;
pub struct AF13;
pub struct AF14;
pub struct AF15;

pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
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

/// Open drain input or output (type state)
pub struct OpenDrain;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

/// GPIO Pin speed selection
pub enum Speed {
    Low = 0,
    Medium = 1,
    High = 2,
    VeryHigh = 3,
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $iopxenr:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $moderi:ident, $oti:ident, $ospeedri:ident, $pupdri:ident, $idri:ident, $bsi:ident, $bri:ident, $afrx:ident, $afrxi:ident, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;

            use hal::digital::{InputPin, OutputPin};
            use stm32::$GPIOX;

            use stm32::RCC;
            use super::{
                Alternate, Floating, GpioExt, Input, OpenDrain, Output, Speed,
                PullDown, PullUp, PushPull, AF0, AF1, AF2, AF3, AF4, AF5, AF6, AF7, AF8, AF9, AF10,
                AF11, AF12, AF13, AF14, AF15
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
                    // NOTE(unsafe) This executes only during initialisation
                    let rcc = unsafe { &(*RCC::ptr()) };
                    rcc.ahb1enr.modify(|_, w| w.$iopxenr().set_bit());

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

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                fn set_high(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << self.i)) }
                }

                fn set_low(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
                }
            }

            impl<MODE> InputPin for $PXx<Output<MODE>> {
                fn is_high(&self) -> bool {
                    !self.is_low()
                }

                fn is_low(&self) -> bool {
                    let gpio = unsafe { &(*$GPIOX::ptr()) };
                    // NOTE(unsafe) atomic read with no side effects
                    gpio.idr.read().bits() & (1 << self.i) == 0
                }
            }

            impl<MODE> InputPin for $PXx<Input<MODE>> {
                fn is_high(&self) -> bool {
                    !self.is_low()
                }

                fn is_low(&self) -> bool {
                    let gpio = unsafe { &(*$GPIOX::ptr()) };
                    // NOTE(unsafe) atomic read with no side effects
                    gpio.idr.read().bits() & (1 << self.i) == 0
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate in AF0 mode
                    pub fn into_alternate_af0(
                        self,
                    ) -> $PXi<Alternate<AF0>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af0());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF1 mode
                    pub fn into_alternate_af1(
                        self,
                    ) -> $PXi<Alternate<AF1>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af1());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF2 mode
                    pub fn into_alternate_af2(
                        self,
                    ) -> $PXi<Alternate<AF2>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af2());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF3 mode
                    pub fn into_alternate_af3(
                        self,
                    ) -> $PXi<Alternate<AF3>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af3());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF4 mode
                    pub fn into_alternate_af4(
                        self,
                    ) -> $PXi<Alternate<AF4>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af4());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF5 mode
                    pub fn into_alternate_af5(
                        self,
                    ) -> $PXi<Alternate<AF5>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af5());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF6 mode
                    pub fn into_alternate_af6(
                        self,
                    ) -> $PXi<Alternate<AF6>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af6());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF7 mode
                    pub fn into_alternate_af7(
                        self,
                    ) -> $PXi<Alternate<AF7>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af7());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF8 mode
                    pub fn into_alternate_af8(
                        self,
                    ) -> $PXi<Alternate<AF8>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af8());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF9 mode
                    pub fn into_alternate_af9(
                        self,
                    ) -> $PXi<Alternate<AF9>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af9());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF10 mode
                    pub fn into_alternate_af10(
                        self,
                    ) -> $PXi<Alternate<AF10>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af10());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF11 mode
                    pub fn into_alternate_af11(
                        self,
                    ) -> $PXi<Alternate<AF11>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af11());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF12 mode
                    pub fn into_alternate_af12(
                        self,
                    ) -> $PXi<Alternate<AF12>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af12());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF13 mode
                    pub fn into_alternate_af13(
                        self,
                    ) -> $PXi<Alternate<AF13>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af13());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF14 mode
                    pub fn into_alternate_af14(
                        self,
                    ) -> $PXi<Alternate<AF14>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af14());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF15 mode
                    pub fn into_alternate_af15(
                        self,
                    ) -> $PXi<Alternate<AF15>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().alternate());
                        gpio.$afrx.modify(|_, w| w.$afrxi().af15());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self,
                    ) -> $PXi<Input<Floating>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().input());
                        gpio.pupdr.modify(|_, w| w.$pupdri().floating());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self,
                    ) -> $PXi<Input<PullDown>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().input());
                        gpio.pupdr.modify(|_, w| w.$pupdri().pull_down());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self,
                    ) -> $PXi<Input<PullUp>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().input());
                        gpio.pupdr.modify(|_, w| w.$pupdri().pull_up());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(
                        self,
                    ) -> $PXi<Output<OpenDrain>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().output());
                        gpio.pupdr.modify(|_, w| w.$pupdri().floating());
                        gpio.otyper.modify(|_, w| w.$oti().open_drain());

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self,
                    ) -> $PXi<Output<PushPull>> {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.moder.modify(|_, w| w.$moderi().output());
                        gpio.pupdr.modify(|_, w| w.$pupdri().floating());
                        gpio.otyper.modify(|_, w| w.$oti().push_pull());

                        $PXi { _mode: PhantomData }
                    }

                    /// Set pin speed
                    pub fn set_speed(self, speed: Speed) -> Self {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.ospeedr.modify(|_, w| w.$ospeedri().bits(speed as u8));

                        self
                    }
                }

                impl $PXi<Output<OpenDrain>> {
                    /// Enables / disables the internal pull up
                    pub fn internal_pull_up(&mut self, on: bool) {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.pupdr.modify(|_, w|
                            if on { w.$pupdri().pull_up() }
                            else  { w.$pupdri().floating() }
                        );
                    }
                }

                impl<MODE> $PXi<Alternate<MODE>> {
                    /// Enables / disables the internal pull up
                    pub fn internal_pull_up(self, on: bool) -> Self {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.pupdr.modify(|_, w|
                            if on { w.$pupdri().pull_up() }
                            else  { w.$pupdri().floating() }
                        );

                        self
                    }
                }

                impl<MODE> $PXi<Alternate<MODE>> {
                    /// Turns pin alternate configuration pin into open drain
                    pub fn set_open_drain(self) -> Self {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        gpio.otyper.modify(|_, w| w.$oti().open_drain());

                        self
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Output<MODE>> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set_high(&mut self) {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        // NOTE(unsafe) atomic write to a stateless register
                        gpio.bsrr.write(|w| w.$bsi().set());
                    }

                    fn set_low(&mut self) {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        // NOTE(unsafe) atomic write to a stateless register
                        gpio.bsrr.write(|w| w.$bri().reset());
                    }
                }

                impl<MODE> InputPin for $PXi<Output<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        // NOTE(unsafe) atomic read with no side effects
                        gpio.idr.read().$idri().is_low()
                    }
                }

                impl<MODE> $PXi<Input<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Input<MODE>> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        let gpio = unsafe { &(*$GPIOX::ptr()) };
                        // NOTE(unsafe) atomic read with no side effects
                        gpio.idr.read().$idri().is_low()
                    }
                }
            )+

                impl<TYPE> $PXx<TYPE> {
                    pub fn get_id (&self) -> u8
                    {
                        self.i
                    }
                }
        }
    }
}

gpio!(GPIOA, gpioa, gpioaen, PA, [
    PA0: (pa0, 0, moder0, ot0, ospeedr0, pupdr0, idr0, bs0, br0, afrl, afrl0, Input<Floating>),
    PA1: (pa1, 1, moder1, ot1, ospeedr1, pupdr1, idr1, bs1, br1, afrl, afrl1, Input<Floating>),
    PA2: (pa2, 2, moder2, ot2, ospeedr2, pupdr2, idr2, bs2, br2, afrl, afrl2, Input<Floating>),
    PA3: (pa3, 3, moder3, ot3, ospeedr3, pupdr3, idr3, bs3, br3, afrl, afrl3, Input<Floating>),
    PA4: (pa4, 4, moder4, ot4, ospeedr4, pupdr4, idr4, bs4, br4, afrl, afrl4, Input<Floating>),
    PA5: (pa5, 5, moder5, ot5, ospeedr5, pupdr5, idr5, bs5, br5, afrl, afrl5, Input<Floating>),
    PA6: (pa6, 6, moder6, ot6, ospeedr6, pupdr6, idr6, bs6, br6, afrl, afrl6, Input<Floating>),
    PA7: (pa7, 7, moder7, ot7, ospeedr7, pupdr7, idr7, bs7, br7, afrl, afrl7, Input<Floating>),
    PA8: (pa8, 8, moder8, ot8, ospeedr8, pupdr8, idr8, bs8, br8, afrh, afrh8, Input<Floating>),
    PA9: (pa9, 9, moder9, ot9, ospeedr9, pupdr9, idr9, bs9, br9, afrh, afrh9, Input<Floating>),
    PA10: (pa10, 10, moder10, ot10, ospeedr10, pupdr10, idr10, bs10, br10, afrh, afrh10, Input<Floating>),
    PA11: (pa11, 11, moder11, ot11, ospeedr11, pupdr11, idr11, bs11, br11, afrh, afrh11, Input<Floating>),
    PA12: (pa12, 12, moder12, ot12, ospeedr12, pupdr12, idr12, bs12, br12, afrh, afrh12, Input<Floating>),
    PA13: (pa13, 13, moder13, ot13, ospeedr13, pupdr13, idr13, bs13, br13, afrh, afrh13, Input<Floating>),
    PA14: (pa14, 14, moder14, ot14, ospeedr14, pupdr14, idr14, bs14, br14, afrh, afrh14, Input<Floating>),
    PA15: (pa15, 15, moder15, ot15, ospeedr15, pupdr15, idr15, bs15, br15, afrh, afrh15, Input<Floating>),
]);

gpio!(GPIOB, gpiob, gpioben, PB, [
    PB0: (pb0, 0, moder0, ot0, ospeedr0, pupdr0, idr0, bs0, br0, afrl, afrl0, Input<Floating>),
    PB1: (pb1, 1, moder1, ot1, ospeedr1, pupdr1, idr1, bs1, br1, afrl, afrl1, Input<Floating>),
    PB2: (pb2, 2, moder2, ot2, ospeedr2, pupdr2, idr2, bs2, br2, afrl, afrl2, Input<Floating>),
    PB3: (pb3, 3, moder3, ot3, ospeedr3, pupdr3, idr3, bs3, br3, afrl, afrl3, Input<Floating>),
    PB4: (pb4, 4, moder4, ot4, ospeedr4, pupdr4, idr4, bs4, br4, afrl, afrl4, Input<Floating>),
    PB5: (pb5, 5, moder5, ot5, ospeedr5, pupdr5, idr5, bs5, br5, afrl, afrl5, Input<Floating>),
    PB6: (pb6, 6, moder6, ot6, ospeedr6, pupdr6, idr6, bs6, br6, afrl, afrl6, Input<Floating>),
    PB7: (pb7, 7, moder7, ot7, ospeedr7, pupdr7, idr7, bs7, br7, afrl, afrl7, Input<Floating>),
    PB8: (pb8, 8, moder8, ot8, ospeedr8, pupdr8, idr8, bs8, br8, afrh, afrh8, Input<Floating>),
    PB9: (pb9, 9, moder9, ot9, ospeedr9, pupdr9, idr9, bs9, br9, afrh, afrh9, Input<Floating>),
    PB10: (pb10, 10, moder10, ot10, ospeedr10, pupdr10, idr10, bs10, br10, afrh, afrh10, Input<Floating>),
    PB11: (pb11, 11, moder11, ot11, ospeedr11, pupdr11, idr11, bs11, br11, afrh, afrh11, Input<Floating>),
    PB12: (pb12, 12, moder12, ot12, ospeedr12, pupdr12, idr12, bs12, br12, afrh, afrh12, Input<Floating>),
    PB13: (pb13, 13, moder13, ot13, ospeedr13, pupdr13, idr13, bs13, br13, afrh, afrh13, Input<Floating>),
    PB14: (pb14, 14, moder14, ot14, ospeedr14, pupdr14, idr14, bs14, br14, afrh, afrh14, Input<Floating>),
    PB15: (pb15, 15, moder15, ot15, ospeedr15, pupdr15, idr15, bs15, br15, afrh, afrh15, Input<Floating>),
]);

gpio!(GPIOC, gpioc, gpiocen, PC, [
    PC0: (pc0, 0, moder0, ot0, ospeedr0, pupdr0, idr0, bs0, br0, afrl, afrl0, Input<Floating>),
    PC1: (pc1, 1, moder1, ot1, ospeedr1, pupdr1, idr1, bs1, br1, afrl, afrl1, Input<Floating>),
    PC2: (pc2, 2, moder2, ot2, ospeedr2, pupdr2, idr2, bs2, br2, afrl, afrl2, Input<Floating>),
    PC3: (pc3, 3, moder3, ot3, ospeedr3, pupdr3, idr3, bs3, br3, afrl, afrl3, Input<Floating>),
    PC4: (pc4, 4, moder4, ot4, ospeedr4, pupdr4, idr4, bs4, br4, afrl, afrl4, Input<Floating>),
    PC5: (pc5, 5, moder5, ot5, ospeedr5, pupdr5, idr5, bs5, br5, afrl, afrl5, Input<Floating>),
    PC6: (pc6, 6, moder6, ot6, ospeedr6, pupdr6, idr6, bs6, br6, afrl, afrl6, Input<Floating>),
    PC7: (pc7, 7, moder7, ot7, ospeedr7, pupdr7, idr7, bs7, br7, afrl, afrl7, Input<Floating>),
    PC8: (pc8, 8, moder8, ot8, ospeedr8, pupdr8, idr8, bs8, br8, afrh, afrh8, Input<Floating>),
    PC9: (pc9, 9, moder9, ot9, ospeedr9, pupdr9, idr9, bs9, br9, afrh, afrh9, Input<Floating>),
    PC10: (pc10, 10, moder10, ot10, ospeedr10, pupdr10, idr10, bs10, br10, afrh, afrh10, Input<Floating>),
    PC11: (pc11, 11, moder11, ot11, ospeedr11, pupdr11, idr11, bs11, br11, afrh, afrh11, Input<Floating>),
    PC12: (pc12, 12, moder12, ot12, ospeedr12, pupdr12, idr12, bs12, br12, afrh, afrh12, Input<Floating>),
    PC13: (pc13, 13, moder13, ot13, ospeedr13, pupdr13, idr13, bs13, br13, afrh, afrh13, Input<Floating>),
    PC14: (pc14, 14, moder14, ot14, ospeedr14, pupdr14, idr14, bs14, br14, afrh, afrh14, Input<Floating>),
    PC15: (pc15, 15, moder15, ot15, ospeedr15, pupdr15, idr15, bs15, br15, afrh, afrh15, Input<Floating>),
]);

gpio!(GPIOD, gpiod, gpioden, PD, [
    PD0: (pd0, 0, moder0, ot0, ospeedr0, pupdr0, idr0, bs0, br0, afrl, afrl0, Input<Floating>),
    PD1: (pd1, 1, moder1, ot1, ospeedr1, pupdr1, idr1, bs1, br1, afrl, afrl1, Input<Floating>),
    PD2: (pd2, 2, moder2, ot2, ospeedr2, pupdr2, idr2, bs2, br2, afrl, afrl2, Input<Floating>),
    PD3: (pd3, 3, moder3, ot3, ospeedr3, pupdr3, idr3, bs3, br3, afrl, afrl3, Input<Floating>),
    PD4: (pd4, 4, moder4, ot4, ospeedr4, pupdr4, idr4, bs4, br4, afrl, afrl4, Input<Floating>),
    PD5: (pd5, 5, moder5, ot5, ospeedr5, pupdr5, idr5, bs5, br5, afrl, afrl5, Input<Floating>),
    PD6: (pd6, 6, moder6, ot6, ospeedr6, pupdr6, idr6, bs6, br6, afrl, afrl6, Input<Floating>),
    PD7: (pd7, 7, moder7, ot7, ospeedr7, pupdr7, idr7, bs7, br7, afrl, afrl7, Input<Floating>),
    PD8: (pd8, 8, moder8, ot8, ospeedr8, pupdr8, idr8, bs8, br8, afrh, afrh8, Input<Floating>),
    PD9: (pd9, 9, moder9, ot9, ospeedr9, pupdr9, idr9, bs9, br9, afrh, afrh9, Input<Floating>),
    PD10: (pd10, 10, moder10, ot10, ospeedr10, pupdr10, idr10, bs10, br10, afrh, afrh10, Input<Floating>),
    PD11: (pd11, 11, moder11, ot11, ospeedr11, pupdr11, idr11, bs11, br11, afrh, afrh11, Input<Floating>),
    PD12: (pd12, 12, moder12, ot12, ospeedr12, pupdr12, idr12, bs12, br12, afrh, afrh12, Input<Floating>),
    PD13: (pd13, 13, moder13, ot13, ospeedr13, pupdr13, idr13, bs13, br13, afrh, afrh13, Input<Floating>),
    PD14: (pd14, 14, moder14, ot14, ospeedr14, pupdr14, idr14, bs14, br14, afrh, afrh14, Input<Floating>),
    PD15: (pd15, 15, moder15, ot15, ospeedr15, pupdr15, idr15, bs15, br15, afrh, afrh15, Input<Floating>),
]);

gpio!(GPIOE, gpioe, gpioeen, PE, [
    PE0: (pe0, 0, moder0, ot0, ospeedr0, pupdr0, idr0, bs0, br0, afrl, afrl0, Input<Floating>),
    PE1: (pe1, 1, moder1, ot1, ospeedr1, pupdr1, idr1, bs1, br1, afrl, afrl1, Input<Floating>),
    PE2: (pe2, 2, moder2, ot2, ospeedr2, pupdr2, idr2, bs2, br2, afrl, afrl2, Input<Floating>),
    PE3: (pe3, 3, moder3, ot3, ospeedr3, pupdr3, idr3, bs3, br3, afrl, afrl3, Input<Floating>),
    PE4: (pe4, 4, moder4, ot4, ospeedr4, pupdr4, idr4, bs4, br4, afrl, afrl4, Input<Floating>),
    PE5: (pe5, 5, moder5, ot5, ospeedr5, pupdr5, idr5, bs5, br5, afrl, afrl5, Input<Floating>),
    PE6: (pe6, 6, moder6, ot6, ospeedr6, pupdr6, idr6, bs6, br6, afrl, afrl6, Input<Floating>),
    PE7: (pe7, 7, moder7, ot7, ospeedr7, pupdr7, idr7, bs7, br7, afrl, afrl7, Input<Floating>),
    PE8: (pe8, 8, moder8, ot8, ospeedr8, pupdr8, idr8, bs8, br8, afrh, afrh8, Input<Floating>),
    PE9: (pe9, 9, moder9, ot9, ospeedr9, pupdr9, idr9, bs9, br9, afrh, afrh9, Input<Floating>),
    PE10: (pe10, 10, moder10, ot10, ospeedr10, pupdr10, idr10, bs10, br10, afrh, afrh10, Input<Floating>),
    PE11: (pe11, 11, moder11, ot11, ospeedr11, pupdr11, idr11, bs11, br11, afrh, afrh11, Input<Floating>),
    PE12: (pe12, 12, moder12, ot12, ospeedr12, pupdr12, idr12, bs12, br12, afrh, afrh12, Input<Floating>),
    PE13: (pe13, 13, moder13, ot13, ospeedr13, pupdr13, idr13, bs13, br13, afrh, afrh13, Input<Floating>),
    PE14: (pe14, 14, moder14, ot14, ospeedr14, pupdr14, idr14, bs14, br14, afrh, afrh14, Input<Floating>),
    PE15: (pe15, 15, moder15, ot15, ospeedr15, pupdr15, idr15, bs15, br15, afrh, afrh15, Input<Floating>),
]);

#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
gpio!(GPIOF, gpiof, gpiofen, PF, [
    PF0: (pf0, 0, moder0, ot0, ospeedr0, pupdr0, idr0, bs0, br0, afrl, afrl0, Input<Floating>),
    PF1: (pf1, 1, moder1, ot1, ospeedr1, pupdr1, idr1, bs1, br1, afrl, afrl1, Input<Floating>),
    PF2: (pf2, 2, moder2, ot2, ospeedr2, pupdr2, idr2, bs2, br2, afrl, afrl2, Input<Floating>),
    PF3: (pf3, 3, moder3, ot3, ospeedr3, pupdr3, idr3, bs3, br3, afrl, afrl3, Input<Floating>),
    PF4: (pf4, 4, moder4, ot4, ospeedr4, pupdr4, idr4, bs4, br4, afrl, afrl4, Input<Floating>),
    PF5: (pf5, 5, moder5, ot5, ospeedr5, pupdr5, idr5, bs5, br5, afrl, afrl5, Input<Floating>),
    PF6: (pf6, 6, moder6, ot6, ospeedr6, pupdr6, idr6, bs6, br6, afrl, afrl6, Input<Floating>),
    PF7: (pf7, 7, moder7, ot7, ospeedr7, pupdr7, idr7, bs7, br7, afrl, afrl7, Input<Floating>),
    PF8: (pf8, 8, moder8, ot8, ospeedr8, pupdr8, idr8, bs8, br8, afrh, afrh8, Input<Floating>),
    PF9: (pf9, 9, moder9, ot9, ospeedr9, pupdr9, idr9, bs9, br9, afrh, afrh9, Input<Floating>),
    PF10: (pf10, 10, moder10, ot10, ospeedr10, pupdr10, idr10, bs10, br10, afrh, afrh10, Input<Floating>),
    PF11: (pf11, 11, moder11, ot11, ospeedr11, pupdr11, idr11, bs11, br11, afrh, afrh11, Input<Floating>),
    PF12: (pf12, 12, moder12, ot12, ospeedr12, pupdr12, idr12, bs12, br12, afrh, afrh12, Input<Floating>),
    PF13: (pf13, 13, moder13, ot13, ospeedr13, pupdr13, idr13, bs13, br13, afrh, afrh13, Input<Floating>),
    PF14: (pf14, 14, moder14, ot14, ospeedr14, pupdr14, idr14, bs14, br14, afrh, afrh14, Input<Floating>),
    PF15: (pf15, 15, moder15, ot15, ospeedr15, pupdr15, idr15, bs15, br15, afrh, afrh15, Input<Floating>),
]);

#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429"))]
gpio!(GPIOG, gpiog, gpiogen, PG, [
    PG0: (pg0, 0, moder0, ot0, ospeedr0, pupdr0, idr0, bs0, br0, afrl, afrl0, Input<Floating>),
    PG1: (pg1, 1, moder1, ot1, ospeedr1, pupdr1, idr1, bs1, br1, afrl, afrl1, Input<Floating>),
    PG2: (pg2, 2, moder2, ot2, ospeedr2, pupdr2, idr2, bs2, br2, afrl, afrl2, Input<Floating>),
    PG3: (pg3, 3, moder3, ot3, ospeedr3, pupdr3, idr3, bs3, br3, afrl, afrl3, Input<Floating>),
    PG4: (pg4, 4, moder4, ot4, ospeedr4, pupdr4, idr4, bs4, br4, afrl, afrl4, Input<Floating>),
    PG5: (pg5, 5, moder5, ot5, ospeedr5, pupdr5, idr5, bs5, br5, afrl, afrl5, Input<Floating>),
    PG6: (pg6, 6, moder6, ot6, ospeedr6, pupdr6, idr6, bs6, br6, afrl, afrl6, Input<Floating>),
    PG7: (pg7, 7, moder7, ot7, ospeedr7, pupdr7, idr7, bs7, br7, afrl, afrl7, Input<Floating>),
    PG8: (pg8, 8, moder8, ot8, ospeedr8, pupdr8, idr8, bs8, br8, afrh, afrh8, Input<Floating>),
    PG9: (pg9, 9, moder9, ot9, ospeedr9, pupdr9, idr9, bs9, br9, afrh, afrh9, Input<Floating>),
    PG10: (pg10, 10, moder10, ot10, ospeedr10, pupdr10, idr10, bs10, br10, afrh, afrh10, Input<Floating>),
    PG11: (pg11, 11, moder11, ot11, ospeedr11, pupdr11, idr11, bs11, br11, afrh, afrh11, Input<Floating>),
    PG12: (pg12, 12, moder12, ot12, ospeedr12, pupdr12, idr12, bs12, br12, afrh, afrh12, Input<Floating>),
    PG13: (pg13, 13, moder13, ot13, ospeedr13, pupdr13, idr13, bs13, br13, afrh, afrh13, Input<Floating>),
    PG14: (pg14, 14, moder14, ot14, ospeedr14, pupdr14, idr14, bs14, br14, afrh, afrh14, Input<Floating>),
    PG15: (pg15, 15, moder15, ot15, ospeedr15, pupdr15, idr15, bs15, br15, afrh, afrh15, Input<Floating>),
]);

#[cfg(any(feature = "stm32f407", feature = "stm32f412", feature = "stm32f429", feature = "stm32f411"))]
gpio!(GPIOH, gpioh, gpiohen, PH, [
    PH0: (ph0, 0, moder0, ot0, ospeedr0, pupdr0, idr0, bs0, br0, afrl, afrl0, Input<Floating>),
    PH1: (ph1, 1, moder1, ot1, ospeedr1, pupdr1, idr1, bs1, br1, afrl, afrl1, Input<Floating>),
    PH2: (ph2, 2, moder2, ot2, ospeedr2, pupdr2, idr2, bs2, br2, afrl, afrl2, Input<Floating>),
    PH3: (ph3, 3, moder3, ot3, ospeedr3, pupdr3, idr3, bs3, br3, afrl, afrl3, Input<Floating>),
    PH4: (ph4, 4, moder4, ot4, ospeedr4, pupdr4, idr4, bs4, br4, afrl, afrl4, Input<Floating>),
    PH5: (ph5, 5, moder5, ot5, ospeedr5, pupdr5, idr5, bs5, br5, afrl, afrl5, Input<Floating>),
    PH6: (ph6, 6, moder6, ot6, ospeedr6, pupdr6, idr6, bs6, br6, afrl, afrl6, Input<Floating>),
    PH7: (ph7, 7, moder7, ot7, ospeedr7, pupdr7, idr7, bs7, br7, afrl, afrl7, Input<Floating>),
    PH8: (ph8, 8, moder8, ot8, ospeedr8, pupdr8, idr8, bs8, br8, afrh, afrh8, Input<Floating>),
    PH9: (ph9, 9, moder9, ot9, ospeedr9, pupdr9, idr9, bs9, br9, afrh, afrh9, Input<Floating>),
    PH10: (ph10, 10, moder10, ot10, ospeedr10, pupdr10, idr10, bs10, br10, afrh, afrh10, Input<Floating>),
    PH11: (ph11, 11, moder11, ot11, ospeedr11, pupdr11, idr11, bs11, br11, afrh, afrh11, Input<Floating>),
    PH12: (ph12, 12, moder12, ot12, ospeedr12, pupdr12, idr12, bs12, br12, afrh, afrh12, Input<Floating>),
    PH13: (ph13, 13, moder13, ot13, ospeedr13, pupdr13, idr13, bs13, br13, afrh, afrh13, Input<Floating>),
    PH14: (ph14, 14, moder14, ot14, ospeedr14, pupdr14, idr14, bs14, br14, afrh, afrh14, Input<Floating>),
    PH15: (ph15, 15, moder15, ot15, ospeedr15, pupdr15, idr15, bs15, br15, afrh, afrh15, Input<Floating>),
]);

#[cfg(any(feature = "stm32f407", feature = "stm32f429"))]
gpio!(GPIOI, gpioi, gpioien, PI, [
    PI0: (pi0, 0, moder0, ot0, ospeedr0, pupdr0, idr0, bs0, br0, afrl, afrl0, Input<Floating>),
    PI1: (pi1, 1, moder1, ot1, ospeedr1, pupdr1, idr1, bs1, br1, afrl, afrl1, Input<Floating>),
    PI2: (pi2, 2, moder2, ot2, ospeedr2, pupdr2, idr2, bs2, br2, afrl, afrl2, Input<Floating>),
    PI3: (pi3, 3, moder3, ot3, ospeedr3, pupdr3, idr3, bs3, br3, afrl, afrl3, Input<Floating>),
    PI4: (pi4, 4, moder4, ot4, ospeedr4, pupdr4, idr4, bs4, br4, afrl, afrl4, Input<Floating>),
    PI5: (pi5, 5, moder5, ot5, ospeedr5, pupdr5, idr5, bs5, br5, afrl, afrl5, Input<Floating>),
    PI6: (pi6, 6, moder6, ot6, ospeedr6, pupdr6, idr6, bs6, br6, afrl, afrl6, Input<Floating>),
    PI7: (pi7, 7, moder7, ot7, ospeedr7, pupdr7, idr7, bs7, br7, afrl, afrl7, Input<Floating>),
    PI8: (pi8, 8, moder8, ot8, ospeedr8, pupdr8, idr8, bs8, br8, afrh, afrh8, Input<Floating>),
    PI9: (pi9, 9, moder9, ot9, ospeedr9, pupdr9, idr9, bs9, br9, afrh, afrh9, Input<Floating>),
    PI10: (pi10, 10, moder10, ot10, ospeedr10, pupdr10, idr10, bs10, br10, afrh, afrh10, Input<Floating>),
    PI11: (pi11, 11, moder11, ot11, ospeedr11, pupdr11, idr11, bs11, br11, afrh, afrh11, Input<Floating>),
    PI12: (pi12, 12, moder12, ot12, ospeedr12, pupdr12, idr12, bs12, br12, afrh, afrh12, Input<Floating>),
    PI13: (pi13, 13, moder13, ot13, ospeedr13, pupdr13, idr13, bs13, br13, afrh, afrh13, Input<Floating>),
    PI14: (pi14, 14, moder14, ot14, ospeedr14, pupdr14, idr14, bs14, br14, afrh, afrh14, Input<Floating>),
    PI15: (pi15, 15, moder15, ot15, ospeedr15, pupdr15, idr15, bs15, br15, afrh, afrh15, Input<Floating>),
]);

/*
gpio!(GPIOJ, gpioj, gpiojen, PJ, [
    PJ0: (pj0, 0, Input<Floating>),
    PJ1: (pj1, 1, Input<Floating>),
    PJ2: (pj2, 2, Input<Floating>),
    PJ3: (pj3, 3, Input<Floating>),
    PJ4: (pj4, 4, Input<Floating>),
    PJ5: (pj5, 5, Input<Floating>),
    PJ6: (pj6, 6, Input<Floating>),
    PJ7: (pj7, 7, Input<Floating>),
    PJ8: (pj8, 8, Input<Floating>),
    PJ9: (pj9, 9, Input<Floating>),
    PJ10: (pj10, 10, Input<Floating>),
    PJ11: (pj11, 11, Input<Floating>),
    PJ12: (pj12, 12, Input<Floating>),
    PJ13: (pj13, 13, Input<Floating>),
    PJ14: (pj14, 14, Input<Floating>),
    PJ15: (pj15, 15, Input<Floating>),
]);

gpio!(GPIOK, gpiok, gpioken, PK, [
    PK0: (pk0, 0, Input<Floating>),
    PK1: (pk1, 1, Input<Floating>),
    PK2: (pk2, 2, Input<Floating>),
    PK3: (pk3, 3, Input<Floating>),
    PK4: (pk4, 4, Input<Floating>),
    PK5: (pk5, 5, Input<Floating>),
    PK6: (pk6, 6, Input<Floating>),
    PK7: (pk7, 7, Input<Floating>),
]);
*/
