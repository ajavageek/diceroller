mod droller;

use crate::droller::damage::NormalDamageDice;

fn main() {
    let one_die = NormalDamageDice::new(1);
    let damage = one_die.roll();
    println!("damage: {}", damage);
}
