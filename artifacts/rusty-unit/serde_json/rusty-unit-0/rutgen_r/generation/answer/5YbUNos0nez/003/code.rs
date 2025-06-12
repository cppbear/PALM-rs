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
fn test_positive_integer_as_i128() {
    let num = Number { n: N::PosInt(100) };
    assert_eq!(num.as_i128(), Some(100 as i128));
}

#[test]
fn test_large_positive_integer_as_i128() {
    let num = Number { n: N::PosInt(1_000_000_000) };
    assert_eq!(num.as_i128(), Some(1_000_000_000 as i128));
}

#[test]
fn test_negative_integer_as_i128() {
    let num = Number { n: N::NegInt(-100) };
    assert_eq!(num.as_i128(), Some(-100 as i128));
}

#[test]
fn test_float_as_i128() {
    let num = Number { n: N::Float(100.0) };
    assert_eq!(num.as_i128(), None);
}

