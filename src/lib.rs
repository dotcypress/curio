#![no_std]

use hal::gpio::SignalEdge;
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
        tim1: TIM1,
        tim16: TIM16,
        spi: SPI1,
        i2c_dev: I2C1,
        i2c_config: i2c::Config,
        exti: &mut EXTI,
        rcc: &mut Rcc,
    ) -> Self {
        let pins = Pins::new(gpioa, gpiob, gpioc, rcc);

        let mut tim16 = tim16.pwm(100.kHz(), rcc);
        tim16.listen();

        let mut lcd_backlight = tim16.bind_pin(pins.lcd_backlight);
        lcd_backlight.enable();
        lcd_backlight.set_duty(0);

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

        let i2c = i2c_dev.i2c(pins.i2c_sda, pins.i2c_clk, i2c_config, rcc);

        let tim1 = delay.release();
        let mut exti = exti;

        let ir_carrier_tim = tim1.pwm(IR_CARRIER_FREQUENCY.Hz(), rcc);
        let rx = pins.ir_rx.listen(SignalEdge::All, &mut exti);
        let ir = IrTransceiver::new(tim16, ir_carrier_tim, pins.ir_tx, rx);

        exti.wakeup(hal::exti::Event::GPIO2);
        let btn_a = pins.btn_a.listen(SignalEdge::Falling, &mut exti);
        let btn_b = pins.btn_b.listen(SignalEdge::Falling, &mut exti);
        let control = Control::new(btn_a, btn_b, pins.thumb_x, pins.thumb_y, adc, rcc);

        Self {
            display,
            ir,
            i2c,
            control,
        }
    }
}
