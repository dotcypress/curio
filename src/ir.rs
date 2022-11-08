use core::convert::Infallible;

use crate::*;
use hal::timer::pwm::{Pwm, PwmPin};
use hal::timer::Channel3;
use infrared::protocol::nec::NecCommand;
use infrared::protocol::*;
use infrared::receiver::Error;
use infrared::sender::Sender;

pub const IR_SAMPLE_FREQUENCY: u32 = 100_000;
pub const IR_CARRIER_FREQUENCY: u32 = 38_000;

pub struct IrTransceiver {
    ir_tim: Pwm<TIM16>,
    tx: Sender<PwmPin<TIM1, Channel3>, { IR_SAMPLE_FREQUENCY }, 128>,
    rx: Receiver<Nec, IrRx>,
    ts: u32,
    event_ts: u32,
}

impl IrTransceiver {
    pub fn new(ir_tim: Pwm<TIM16>, carrier_tim: Pwm<TIM1>, tx_pin: IrTx, rx_pin: IrRx) -> Self {
        let mut tx_pin = carrier_tim.bind_pin(tx_pin);
        tx_pin.set_duty(tx_pin.get_max_duty() / 2);

        let rx = infrared::receiver::Receiver::with_input(IR_SAMPLE_FREQUENCY, rx_pin);
        let tx = infrared::sender::Sender::new(tx_pin);

        Self {
            ir_tim,
            tx,
            rx,
            ts: 0,
            event_ts: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tx.tick();
        self.ts = self.ts.wrapping_add(1);
        self.ir_tim.clear_irq();
    }

    pub fn event(&mut self) -> Result<Option<NecCommand>, Error<Infallible>> {
        let dt = self.ts.wrapping_sub(self.event_ts);
        self.event_ts = self.ts;
        self.rx.event(dt)
    }

    pub fn send(&mut self, cmd: &NecCommand) {
        self.tx.load::<Nec>(cmd);
    }
}
