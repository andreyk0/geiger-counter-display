use stm32f1xx_hal::stm32::{RCC, TIM1};

pub struct PulseTimer {
    timer: TIM1,
    count: i16,
}

impl PulseTimer {
    pub fn enable(rcc: &mut RCC) -> () {
        rcc.apb2enr.modify(|_, w| w.tim1en().set_bit()); // enable clock
    }

    pub fn new(mut timer: TIM1) -> Self {
        setup_pulse_timer(&mut timer);

        PulseTimer { timer, count: 0 }
    }

    pub fn poll(&mut self) -> i16 {
        let cnt = (self.timer.cnt.read().bits() as i16) >> 2;
        let (diff, _) = cnt.overflowing_sub(self.count);
        self.count = cnt;
        diff
    }
}

fn setup_pulse_timer(tim: &mut TIM1) {
    // NOTE(unsafe) This executes only during initialisation
    /*
       let rcc = unsafe { &(*RCC::ptr()) };
       rcc.apb2enr.modify(|_, w| w.tim1en().set_bit()); // enable clock

       tim.ccmr1_input_mut().modify(|_, w| {
           w.cc1s()
               .ti1() // 01: CC1 channel is configured as input, IC1 is mapped on TI1
               .cc2s()
               .ti2() // 01: CC2 channel is configured as input, IC2 is mapped on TI2
               .ic1f()
               .bits(0b1111) // input capture 1 filter
               .ic2f()
               .bits(0b1111) // input capture 2 filter
       });
       tim.ccer.modify(|_, w| {
           // CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations.
           // 01: inverted/falling edge
           //   The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external
           //    clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode)
           w.cc1p()
               .set_bit() // active low
               .cc1np()
               .clear_bit()
               .cc2p()
               .set_bit() // active low
               .cc2np()
               .clear_bit()
       });
       tim.smcr.modify(|_, w| {
           w.sms().bits(0b011) // Encoder mode3 (resolution X4 on TI1 and TI2): SMS=’011’ in SMCR register.
       });

       tim.cr1.modify(|_, w| w.cen().set_bit()); // enable counter
    */
}
