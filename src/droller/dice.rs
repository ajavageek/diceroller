use paste::paste;

use crate::Die;
use std::num::NonZeroU8;

macro_rules! gen_dice_fn_for {
    ( $( $x:expr ),* ) => {
        paste! {
            $(
            #[allow(dead_code)]
            pub fn [<d$x>]() -> Die {
                Self::new($x)
            }
            )*
        }
    };
}

impl Die {
    pub fn new(faces: u8) -> Die {
        let faces = NonZeroU8::new(faces).unwrap().get();
        Die { faces }
    }
    gen_dice_fn_for![2, 4, 6, 8, 10, 12, 20, 30, 100];
}
