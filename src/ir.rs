use core::convert::Infallible;

use crate::*;
use hal::timer::pwm::{Pwm, PwmPin};
use hal::timer::Channel3;
use infrared::protocol::nec::NecCommand;
use infrared::protocol::*;
use infrared::receiver::Error;
use infrared::sender::Sender;

pub struct IrTransceiver {
    sample_tim: Pwm<TIM16>,
    tx: Sender<PwmPin<TIM1, Channel3>, { IR_SAMPLE_FREQUENCY }, 128>,
    rx: PeriodicPoll<Nec, IrRx>,
}

impl IrTransceiver {
    pub fn new(sample_tim: Pwm<TIM16>, carrier_tim: Pwm<TIM1>, tx_pin: IrTx, rx_pin: IrRx) -> Self {
        let mut tx_pin = carrier_tim.bind_pin(tx_pin);
        tx_pin.set_duty(tx_pin.get_max_duty() / 2);

        let rx = infrared::receiver::PeriodicPoll::with_input(IR_SAMPLE_FREQUENCY, rx_pin);
        let tx = infrared::sender::Sender::new(tx_pin);

        Self { sample_tim, tx, rx }
    }

    pub fn poll(&mut self) -> Result<Option<NecCommand>, Error<Infallible>> {
        self.sample_tim.clear_irq();
        self.tx.tick();
        self.rx.poll()
    }

    pub fn send(&mut self, cmd: &NecCommand) {
        self.tx.load::<Nec>(cmd);
    }
}
