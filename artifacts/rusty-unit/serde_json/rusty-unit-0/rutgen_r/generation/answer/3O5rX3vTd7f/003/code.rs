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
fn test_as_u128_with_small_positive_integer() {
    let num = Number { n: N::PosInt(10) };
    assert_eq!(num.as_u128(), Some(10));
}

#[test]
fn test_as_u128_with_large_positive_integer() {
    let num = Number { n: N::PosInt(1_000_000_000_000_000_000) };
    assert_eq!(num.as_u128(), Some(1_000_000_000_000_000_000));
}

#[test]
fn test_as_u128_with_zero() {
    let num = Number { n: N::PosInt(0) };
    assert_eq!(num.as_u128(), Some(0));
}

#[test]
fn test_as_u128_with_negative_integer() {
    let num = Number { n: N::NegInt(-5) };
    assert_eq!(num.as_u128(), None);
}

#[test]
fn test_as_u128_with_float() {
    let num = Number { n: N::Float(10.5) };
    assert_eq!(num.as_u128(), None);
}

