mod droller;

use crate::droller::damage::{KillingDamageDice, NormalDamageDice};

fn main() {
    let normal_die = NormalDamageDice::new(1);
    let normal_dmg = normal_die.roll();
    println!("normal damage: {}", normal_dmg);
    let killing_die = KillingDamageDice::new(1);
    let killing_dmg = killing_die.roll();
    println!("killing damage: {}", killing_dmg);
    let killing_half = KillingDamageDice::new_and_half(0);
    let killing_half_dmg = killing_half.roll();
    println!("killing damage (half): {}", killing_half_dmg);
    let killing_pip = KillingDamageDice::new_and_pip(0);
    let killing_pip_dmg = killing_pip.roll();
    println!("killing damage (pip): {}", killing_pip_dmg);
}
