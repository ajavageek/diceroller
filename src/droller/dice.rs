use crate::Die;

impl Die {
    pub fn new(faces: u8) -> Die {
        Die { faces }
    }
    pub fn d2() -> Die {
        Self::new(2)
    }
    pub fn d4() -> Die {
        Self::new(4)
    }
    pub fn d6() -> Die {
        Self::new(6)
    }
    pub fn d8() -> Die {
        Self::new(8)
    }
    pub fn d10() -> Die {
        Self::new(10)
    }
    pub fn d12() -> Die {
        Self::new(12)
    }
    pub fn d20() -> Die {
        Self::new(20)
    }
    pub fn d30() -> Die {
        Self::new(30)
    }
    pub fn d100() -> Die {
        Self::new(100)
    }
}
