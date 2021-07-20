mod droller;

use crate::droller::damage::{KillingDamageDice, NormalDamage, NormalDamageDice};

fn main() {
    let normal_die = NormalDamageDice::new(1);
    let normal_dmg = normal_die.roll();
    println!(
        "normal damage: {}",
        normal_dmg.as_any().downcast_ref::<NormalDamage>().unwrap()
    );
    let killing_die = KillingDamageDice::new(1);
    let killing_dmg = killing_die.roll();
    println!(
        "killing damage: {}",
        killing_dmg.as_any().downcast_ref::<NormalDamage>().unwrap()
    );
}
