mod droller;

use crate::droller::Die;

fn main() {
    let d6 = Die::default();
    println!("{}", d6.roll());
}
