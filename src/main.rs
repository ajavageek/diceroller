use rand::Rng;

struct Die {
    faces: u8,
}

impl Die {
    fn roll(self) -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.faces)
    }
}

fn main() {
    let d6 = Die { faces: 6 };
    println!("{}", d6.roll());
}
