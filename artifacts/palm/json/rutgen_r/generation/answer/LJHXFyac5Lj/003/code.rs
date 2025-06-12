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
fn test_as_i64_pos_int_within_bounds() {
    let number = Number { n: N::PosInt(i64::MAX as u64) };
    assert_eq!(number.as_i64(), Some(i64::MAX));
}

#[test]
fn test_as_i64_pos_int_exceeding_bounds() {
    let number = Number { n: N::PosInt(i64::MAX as u64 + 1) };
    assert_eq!(number.as_i64(), None);
}  

#[test]
fn test_as_i64_neg_int() {
    let number = Number { n: N::NegInt(-10) };
    assert_eq!(number.as_i64(), Some(-10));
} 

#[test]
fn test_as_i64_float() {
    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.as_i64(), None);
}

