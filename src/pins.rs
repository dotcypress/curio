use crate::hal::gpio::{gpioa::*, gpiob::*, gpioc::*};
use hal::gpio::{DefaultMode, Floating, Input, OpenDrain, Output, PushPull};
use hal::prelude::*;
use hal::rcc::Rcc;
use hal::stm32::*;

// Qwiic I2C
pub type I2cClk = PB8<Output<OpenDrain>>;
pub type I2cSda = PB9<Output<OpenDrain>>;

// Display
pub type LcdDC = PA4<Output<PushPull>>;
pub type LcdCS = PA11<Output<PushPull>>;
pub type LcdReset = PB0<Output<PushPull>>;
pub type LcdClk = PA5<DefaultMode>;
pub type LcdSda = PA7<DefaultMode>;
pub type LcdBacklight = PA6<DefaultMode>;

// Infared
pub type IrTx = PB6<DefaultMode>;
pub type IrRx = PA12<Input<Floating>>;

// Buttons
pub type ButtonA = PA2<Input<Floating>>;
pub type ButtonB = PA3<Input<Floating>>;

// Thumb
pub type ThumbX = PA1<DefaultMode>;
pub type ThumbY = PA0<DefaultMode>;

// DisplayPower
pub type DisplayPower = PC15<Output<PushPull>>;

// SWD
pub type SwdIo = PA13<DefaultMode>;
pub type SwdClk = PA14<DefaultMode>;

pub struct Pins {
    // SWD
    pub swd_io: SwdIo,
    pub swd_clk: SwdClk,

    // Qwiic I2C
    pub i2c_clk: I2cClk,
    pub i2c_sda: I2cSda,

    // Display
    pub lcd_reset: LcdReset,
    pub lcd_dc: LcdDC,
    pub lcd_cs: LcdCS,
    pub lcd_clk: LcdClk,
    pub lcd_sda: LcdSda,
    pub lcd_backlight: LcdBacklight,

    // Infared
    pub ir_tx: IrTx,
    pub ir_rx: IrRx,

    // Buttons
    pub btn_a: ButtonA,
    pub btn_b: ButtonB,

    // Thumb
    pub thumb_x: ThumbX,
    pub thumb_y: ThumbY,

    // Power
    pub lcd_power: DisplayPower,
}

impl Pins {
    pub fn new(gpioa: GPIOA, gpiob: GPIOB, gpioc: GPIOC, rcc: &mut Rcc) -> Self {
        let port_a = gpioa.split(rcc);
        let port_b = gpiob.split(rcc);
        let port_c = gpioc.split(rcc);

        Self {
            // SWD
            swd_io: port_a.pa13,
            swd_clk: port_a.pa14,

            // Qwiic I2C
            i2c_clk: port_b.pb8.into_open_drain_output_in_state(PinState::High),
            i2c_sda: port_b.pb9.into_open_drain_output_in_state(PinState::High),

            // Display
            lcd_reset: port_b.pb0.into(),
            lcd_dc: port_a.pa4.into(),
            lcd_cs: port_a.pa11.into(),
            lcd_clk: port_a.pa5,
            lcd_sda: port_a.pa7,
            lcd_backlight: port_a.pa6,

            // Infared
            ir_tx: port_b.pb6,
            ir_rx: port_a.pa12.into(),

            // Buttons
            btn_a: port_a.pa2.into(),
            btn_b: port_a.pa3.into(),

            // Thumb
            thumb_y: port_a.pa0,
            thumb_x: port_a.pa1,

            // Display Power
            lcd_power: port_c.pc15.into(),
        }
    }
}
