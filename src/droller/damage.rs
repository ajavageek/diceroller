use crate::droller::Die;
use std::{
    any::Any,
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

pub trait Damage {
    fn stun(self) -> u8;
    fn body(self) -> u8;
    fn as_any(&self) -> &dyn Any;
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
    fn as_any(&self) -> &dyn Any {
        self
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
}

impl KillingDamageDice {
    pub fn new(number: u8) -> KillingDamageDice {
        let number = NonZeroU8::new(number).unwrap().get();
        KillingDamageDice { number }
    }
    pub fn roll(self) -> Box<dyn Damage> {
        let body = (0..self.number)
            .map(|_| Die::default())
            .map(|die| die.roll())
            .sum();
        let mult = max(1, Die::default().roll() - 1);
        Box::new(NormalDamage {
            body,
            stun: body * mult,
        })
    }
}
