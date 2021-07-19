use crate::droller::Die;
use std::{iter::Sum, num::NonZeroU8};

impl Default for Die {
    fn default() -> Self {
        Die::d6()
    }
}

pub struct Damage {
    pub stun: u8,
    pub body: u8,
}

impl Damage {
    fn zero() -> Damage {
        Damage { stun: 0, body: 0 }
    }
}

impl Sum for Damage {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Damage::zero(), |dmg1, dmg2| Damage {
            stun: dmg1.stun + dmg2.stun,
            body: dmg1.body + dmg2.body,
        })
    }
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
        (0..self.number)
            .map(|_| Die::default())
            .map(|die| die.roll())
            .map(|stun| {
                let body = match stun {
                    1 => 0,
                    6 => 2,
                    _ => 1,
                };
                Damage { stun, body }
            })
            .sum()
    }
}
