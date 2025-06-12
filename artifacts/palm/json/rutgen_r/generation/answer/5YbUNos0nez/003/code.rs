// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

struct Number {
    n: N,
}

impl Number {
    pub fn as_i128(&self) -> Option<i128> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as i128),
            N::NegInt(n) => Some(n as i128),
            N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse().ok()
    }
}

#[test]
fn test_as_i128_positive_integer() {
    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.as_i128(), Some(42));
}

#[test]
fn test_as_i128_zero() {
    let number = Number { n: N::PosInt(0) };
    assert_eq!(number.as_i128(), Some(0));
}

#[test]
fn test_as_i128_large_positive_integer() {
    let number = Number { n: N::PosInt(u64::MAX) };
    assert_eq!(number.as_i128(), Some(i128::MAX));
}

