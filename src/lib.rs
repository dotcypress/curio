#![no_std]

use hal::rcc::Rcc;
use hal::stm32::*;
use hal::timer::pwm::PwmPin;
use hal::timer::Channel1;

pub extern crate stm32g0xx_hal as hal;

mod control;
mod display;
mod ir;
mod pins;

pub use control::*;
pub use display::*;
pub use hal::i2c;
pub use hal::prelude::*;
pub use hal::stm32;
pub use infrared::*;
pub use ir::*;
pub use pins::*;

pub type I2cDev = hal::i2c::I2c<I2C1, I2cSda, I2cClk>;

pub const IR_SAMPLE_FREQUENCY: u32 = 20_000;
pub const IR_CARRIER_FREQUENCY: u32 = 38_000;

pub struct Curio {
    pub control: Control,
    pub display: Display,
    pub ir: IrTransceiver,
    pub i2c: I2cDev,
}

impl Curio {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adc: ADC,
        gpioa: GPIOA,
        gpiob: GPIOB,
        gpioc: GPIOC,
        exti: EXTI,
        tim1: TIM1,
        tim16: TIM16,
        spi: SPI1,
        i2c_dev: I2C1,
        i2c_config: i2c::Config,
        rcc: &mut Rcc,
    ) -> Self {
        let pins = Pins::new(gpioa, gpiob, gpioc, rcc);

        let control = Control::new(
            pins.btn_a,
            pins.btn_b,
            pins.thumb_x,
            pins.thumb_y,
            adc,
            exti,
            rcc,
        );

        let i2c = i2c_dev.i2c(pins.i2c_sda, pins.i2c_clk, i2c_config, rcc);

        let mut tim16 = tim16.pwm(IR_SAMPLE_FREQUENCY.Hz(), rcc);
        tim16.listen();

        let mut lcd_backlight = tim16.bind_pin(pins.lcd_backlight);
        lcd_backlight.set_duty(0);
        lcd_backlight.enable();

        let mut delay = tim1.delay(rcc);
        let display = Display::new(
            spi,
            pins.lcd_reset,
            pins.lcd_cs,
            pins.lcd_dc,
            pins.lcd_clk,
            pins.lcd_sda,
            pins.lcd_power,
            lcd_backlight,
            &mut delay,
            rcc,
        );
        let tim1 = delay.release();

        let ir_carrier_tim = tim1.pwm(IR_CARRIER_FREQUENCY.Hz(), rcc);
        let ir = IrTransceiver::new(tim16, ir_carrier_tim, pins.ir_tx, pins.ir_rx);

        Self {
            display,
            ir,
            i2c,
            control,
        }
    }
}
