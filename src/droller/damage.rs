use crate::droller::Die;

impl Default for Die {
    fn default() -> Self {
        Die::d6()
    }
}
