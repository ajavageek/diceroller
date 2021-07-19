use crate::droller::Die;
use std::num::NonZeroU8;

impl Default for Die {
    fn default() -> Self {
        Die::d6()
    }
}

pub struct Damage {
    pub stun: u8,
    pub body: u8,
}

pub struct NormalDamageDice {
    number: u8,
}

impl NormalDamageDice {
    pub fn new(number: u8) -> NormalDamageDice {
        let number = NonZeroU8::new(number).unwrap().get();
        NormalDamageDice { number }
    }
    pub fn roll(self) -> Damage {
        let mut stun = 0;
        let mut body = 0;
        for _ in 0..self.number {
            let die = Die::default();
            let roll = die.roll();
            stun += roll;
            if roll == 1 {
            } else if roll == 6 {
                body += 2
            } else {
                body += 1
            }
        }
        Damage { stun, body }
    }
}
