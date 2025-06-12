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
    pub fn as_i64(&self) -> Option<i64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => {
                if n <= i64::MAX as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::NegInt(n) => Some(n),
            N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse().ok()
    }
}

#[test]
fn test_as_i64_positive_integer_within_bounds() {
    let num = Number { n: N::PosInt(i64::MAX as u64) };
    assert_eq!(num.as_i64(), Some(i64::MAX));
}

#[test]
fn test_as_i64_negative_integer() {
    let num = Number { n: N::NegInt(-123) };
    assert_eq!(num.as_i64(), Some(-123));
}

#[test]
fn test_as_i64_float() {
    let num = Number { n: N::Float(123.45) };
    assert_eq!(num.as_i64(), None);
}

#[test]
fn test_as_i64_positive_integer_out_of_bounds() {
    let num = Number { n: N::PosInt(i64::MAX as u64 + 1) };
    assert_eq!(num.as_i64(), None);
}

