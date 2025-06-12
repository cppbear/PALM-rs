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
    pub fn as_u128(&self) -> Option<u128> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as u128),
            N::NegInt(_) | N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse().ok()
    }
}

#[test]
fn test_as_u128_positive_integer() {
    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.as_u128(), Some(42));
}

#[test]
fn test_as_u128_negative_integer() {
    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.as_u128(), None);
}

#[test]
fn test_as_u128_float() {
    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.as_u128(), None);
}

#[test]
fn test_as_u128_large_positive_integer() {
    let number = Number { n: N::PosInt(u64::MAX) };
    assert_eq!(number.as_u128(), Some(u64::MAX as u128));
}

