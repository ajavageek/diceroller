mod droller;

use crate::droller::Die;

fn main() {
    let d6 = Die { faces: 6 };
    println!("{}", d6.roll());
}
