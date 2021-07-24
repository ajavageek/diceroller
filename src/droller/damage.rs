use crate::droller::Die;
use std::{
    cmp::max,
    fmt::{Display, Formatter},
    iter::Sum,
    num::NonZeroU8,
};

impl Default for Die {
    fn default() -> Self {
        Die::d6()
    }
}

pub trait Damage: Display {
    fn stun(self) -> u8;
    fn body(self) -> u8;
}

#[derive(Copy, Clone)]
pub struct NormalDamage {
    pub stun: u8,
    pub body: u8,
}

impl NormalDamage {
    fn zero() -> NormalDamage {
        NormalDamage { stun: 0, body: 0 }
    }
}

impl Damage for NormalDamage {
    fn stun(self) -> u8 {
        self.stun
    }
    fn body(self) -> u8 {
        self.body
    }
}

impl Sum for NormalDamage {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(NormalDamage::zero(), |dmg1, dmg2| NormalDamage {
            stun: dmg1.stun + dmg2.stun,
            body: dmg1.body + dmg2.body,
        })
    }
}

impl Display for NormalDamage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stun: {}, body: {}", self.stun(), self.body())
    }
}

#[derive(Copy, Clone)]
pub struct KillingDamage {
    pub body: u8,
    pub mult: u8,
}

impl Damage for KillingDamage {
    fn stun(self) -> u8 {
        self.body * self.mult
    }
    fn body(self) -> u8 {
        self.body
    }
}

impl Display for KillingDamage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stun: {}, body: {} (mult: {})", self.stun(), self.body, self.mult)
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
    pub fn roll(self) -> Box<dyn Damage> {
        let damage = (0..self.number)
            .map(|_| Die::default())
            .map(|die| die.roll())
            .map(|stun| {
                let body = match stun {
                    1 => 0,
                    6 => 2,
                    _ => 1,
                };
                NormalDamage { stun, body }
            })
            .sum::<NormalDamage>();
        Box::new(damage)
    }
}

pub struct KillingDamageDice {
    number: u8,
    half: bool,
    pip: bool,
}

impl KillingDamageDice {
    pub fn new(number: u8) -> KillingDamageDice {
        let number = NonZeroU8::new(number).unwrap().get();
        KillingDamageDice {
            number,
            half: false,
            pip: false,
        }
    }
    pub fn new_and_half(number: u8) -> KillingDamageDice {
        KillingDamageDice {
            number,
            half: true,
            pip: false,
        }
    }
    pub fn new_and_pip(number: u8) -> KillingDamageDice {
        KillingDamageDice {
            number,
            half: false,
            pip: true,
        }
    }
    pub fn roll(self) -> Box<dyn Damage> {
        let mut body = (0..self.number)
            .map(|_| Die::default())
            .map(|die| die.roll())
            .sum();
        if self.pip {
            body += 1;
        } else if self.half {
            let d3 = Die::new(3);
            body += d3.roll();
        }
        let mult = max(1, Die::default().roll() - 1);
        Box::new(KillingDamage { body, mult })
    }
}
