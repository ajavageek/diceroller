mod droller;

use crate::droller::damage::{KillingDamageDice, NormalDamageDice};

fn main() {
    let normal_die = NormalDamageDice::new(1);
    let normal_dmg = normal_die.roll();
    println!("damage: {}", normal_dmg);
    let killing_die = KillingDamageDice::new(1);
    let killing_dmg = killing_die.roll();
    println!("damage: {}", killing_dmg);
}
