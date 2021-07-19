pub mod damage;
mod dice;

use rand::Rng;

pub struct Die {
    pub faces: u8,
}

impl Die {
    pub fn roll(self) -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.faces)
    }
}
