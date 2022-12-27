// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend - (quotient * divisor);
    (quotient, remainder)
}


struct EvenIter<T, I: Iterator<Item = T>> {
    inner: I
}

impl<I: Iterator<Item = T>, T> Iterator for EvenIter<T, I> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let temp = self.inner.next();
        let _ = self.inner.next();
        temp
    }
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    EvenIter { inner: iter }

}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
