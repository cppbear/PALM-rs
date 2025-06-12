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
    pub fn is_u64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(_) => true,
            N::NegInt(_) | N::Float(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.as_u64().is_some()
    }
    
    #[cfg(feature = "arbitrary_precision")]
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::PosInt(val) => Some(val),
            _ => None,
        }
    }
}

#[test]
fn test_is_u64_positive_integer() {
    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.is_u64(), true);
}

#[test]
fn test_is_u64_zero() {
    let number = Number { n: N::PosInt(0) };
    assert_eq!(number.is_u64(), true);
}

#[test]
fn test_is_u64_large_positive_integer() {
    let number = Number { n: N::PosInt(u64::MAX) };
    assert_eq!(number.is_u64(), true);
} 

#[test]
fn test_is_u64_negative_integer() {
    let number = Number { n: N::NegInt(-1) };
    assert_eq!(number.is_u64(), false);
}

#[test]
fn test_is_u64_float() {
    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.is_u64(), false);
}

