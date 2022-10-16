use crate::*;
use hal::rcc::Rcc;
use hal::spi::{self, NoMiso};
use hal::timer::delay::Delay;
use klaptik::drivers::st7567::{Command, ST7567};
use klaptik::{Canvas, Point, Rectangle, Size};

pub type SpiDev = spi::Spi<SPI1, (LcdClk, NoMiso, LcdSda)>;
pub type DisplayDriver = ST7567<SpiDev, LcdReset, LcdCS, LcdDC>;
pub type Backlight = PwmPin<TIM16, Channel1>;

pub struct Display {
    driver: DisplayDriver,
    backlight: Backlight,
    power: DisplayPower,
}

impl Display {
    pub const SIZE: Size = Size::new(128, 64);

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        spi_dev: SPI1,
        lcd_reset: LcdReset,
        lcd_cs: LcdCS,
        lcd_dc: LcdDC,
        lcd_clk: LcdClk,
        lcd_sda: LcdSda,
        power: DisplayPower,
        backlight: Backlight,
        delay: &mut Delay<TIM1>,
        rcc: &mut Rcc,
    ) -> Self {
        let mut power = power;
        power.set_high().unwrap();
        delay.delay_ms(10_u32);

        let spi = spi_dev.spi((lcd_clk, NoMiso, lcd_sda), spi::MODE_0, 4.MHz(), rcc);
        let mut driver = ST7567::new(spi, lcd_cs, lcd_dc, lcd_reset);
        driver.set_offset(Point::new(4, 0));
        driver.reset(delay);
        driver
            .link()
            .command(|tx| tx.write(&[Command::SegmentDirectionRev as _]))
            .ok();
        driver.on();
        Display {
            driver,
            backlight,
            power,
        }
    }

    pub fn power_off(&mut self) {
        self.driver.off();
        self.power.set_low().unwrap();
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        let max_duty = self.backlight.get_max_duty() as u32;
        let duty = brightness.clamp(0, 10) as u32 * max_duty / 10;
        self.backlight.set_duty(duty as u16)
    }
}

impl Canvas for Display {
    fn draw(&mut self, bounds: Rectangle, buf: &[u8]) {
        self.driver.draw(bounds, buf);
    }
}
