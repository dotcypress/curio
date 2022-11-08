use crate::*;
use hal::analog::adc;

#[derive(Debug)]
pub enum Button {
    A,
    B,
    Up,
    Right,
    Down,
    Left,
}

pub struct Control {
    adc: adc::Adc,
    vbat: adc::VBat,
    btn_a: ButtonA,
    btn_b: ButtonB,
    thumb_x: ThumbX,
    thumb_y: ThumbY,
}

impl Control {
    pub fn new(
        btn_a: ButtonA,
        btn_b: ButtonB,
        thumb_x: ThumbX,
        thumb_y: ThumbY,
        adc: ADC,
        rcc: &mut Rcc,
    ) -> Self {
        let mut adc = adc.constrain(rcc);
        adc.set_sample_time(adc::SampleTime::T_80);
        adc.set_precision(adc::Precision::B_12);
        adc.set_oversampling_shift(24);
        adc.set_oversampling_ratio(adc::OversamplingRatio::X_8);
        adc.oversampling_enable(true);

        let mut vbat = adc::VBat::new();
        vbat.enable(&mut adc);

        adc.calibrate();
        Self {
            vbat,
            btn_a,
            btn_b,
            adc,
            thumb_x,
            thumb_y,
        }
    }

    pub fn battery_voltage(&mut self) -> u16 {
        self.adc.read_voltage(&mut self.vbat).unwrap_or_default() * 3
    }

    pub fn read_buttons(&mut self) -> Option<Button> {
        if self.btn_a.is_low().unwrap_or_default() {
            return Some(Button::A);
        }
        if self.btn_b.is_low().unwrap_or_default() {
            return Some(Button::B);
        }
        None
    }

    pub fn read_dpad(&mut self) -> Option<Button> {
        let (x, y) = self.read_thumb();
        if x > 32 {
            Some(Button::Right)
        } else if x < -32 {
            Some(Button::Left)
        } else if y > 32 {
            Some(Button::Up)
        } else if y < -32 {
            Some(Button::Down)
        } else {
            None
        }
    }

    pub fn read_thumb(&mut self) -> (i8, i8) {
        (
            self.adc.read(&mut self.thumb_x).unwrap_or(0) as i8 - 63,
            self.adc.read(&mut self.thumb_y).unwrap_or(0) as i8 - 63,
        )
    }
}
