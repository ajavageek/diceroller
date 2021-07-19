use crate::Die;
use std::num::NonZeroU8;

impl Die {
    pub fn new(faces: u8) -> Die {
        let faces = NonZeroU8::new(faces).unwrap().get();
        Die { faces }
    }
    #[allow(dead_code)]
    pub fn d2() -> Die {
        Self::new(2)
    }
    #[allow(dead_code)]
    pub fn d4() -> Die {
        Self::new(4)
    }
    #[allow(dead_code)]
    pub fn d6() -> Die {
        Self::new(6)
    }
    #[allow(dead_code)]
    pub fn d8() -> Die {
        Self::new(8)
    }
    #[allow(dead_code)]
    pub fn d10() -> Die {
        Self::new(10)
    }
    #[allow(dead_code)]
    pub fn d12() -> Die {
        Self::new(12)
    }
    #[allow(dead_code)]
    pub fn d20() -> Die {
        Self::new(20)
    }
    #[allow(dead_code)]
    pub fn d30() -> Die {
        Self::new(30)
    }
    #[allow(dead_code)]
    pub fn d100() -> Die {
        Self::new(100)
    }
}
