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
fn test_as_i64_neg_int() {
    let number = Number { n: N::NegInt(-5) };
    assert_eq!(number.as_i64(), Some(-5));
}

#[test]
fn test_as_i64_large_neg_int() {
    let number = Number { n: N::NegInt(i64::MIN) };
    assert_eq!(number.as_i64(), Some(i64::MIN));
}

