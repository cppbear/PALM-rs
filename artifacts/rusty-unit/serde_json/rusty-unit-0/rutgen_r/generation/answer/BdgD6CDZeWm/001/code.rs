// Answer 0

#[derive(Debug)]
enum N {
    PosInt(i32),
    NegInt(i32),
    Float(f64),
}

struct Number {
    n: N,
}

impl Number {
    pub fn as_f64(&self) -> Option<f64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as f64),
            N::NegInt(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse::<f64>().ok().filter(|float| float.is_finite())
    }
}

#[test]
fn test_as_f64_with_positive_float() {
    let number = Number { n: N::Float(1.23) };
    assert_eq!(number.as_f64(), Some(1.23));
}

#[test]
fn test_as_f64_with_negative_float() {
    let number = Number { n: N::Float(-4.56) };
    assert_eq!(number.as_f64(), Some(-4.56));
}

#[test]
fn test_as_f64_with_zero_float() {
    let number = Number { n: N::Float(0.0) };
    assert_eq!(number.as_f64(), Some(0.0));
}

#[test]
fn test_as_f64_with_large_float() {
    let number = Number { n: N::Float(1e10) };
    assert_eq!(number.as_f64(), Some(1e10));
}

#[test]
fn test_as_f64_with_small_float() {
    let number = Number { n: N::Float(1e-10) };
    assert_eq!(number.as_f64(), Some(1e-10));
}

#[test]
fn test_as_f64_with_nan_float() {
    let number = Number { n: N::Float(f64::NAN) };
    assert_eq!(number.as_f64(), None);
}

#[test]
fn test_as_f64_with_infinite_float() {
    let number = Number { n: N::Float(f64::INFINITY) };
    assert_eq!(number.as_f64(), None);
}

#[test]
fn test_as_f64_with_negative_infinite_float() {
    let number = Number { n: N::Float(f64::NEG_INFINITY) };
    assert_eq!(number.as_f64(), None);
}

