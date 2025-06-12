// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u32),
    NegInt(i32),
    Float(f64),
}

struct Number {
    n: N,
}

impl Number {
    pub(crate) fn as_f32(&self) -> Option<f32> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as f32),
            N::NegInt(n) => Some(n as f32),
            N::Float(n) => Some(n as f32),
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse::<f32>().ok().filter(|float| float.is_finite())
    }
}

#[test]
fn test_as_f32_pos_int() {
    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.as_f32(), Some(42.0));
}

#[test]
fn test_as_f32_neg_int() {
    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.as_f32(), Some(-42.0));
}

#[test]
fn test_as_f32_float() {
    let number = Number { n: N::Float(42.0) };
    assert_eq!(number.as_f32(), Some(42.0));
}

#[test]
fn test_as_f32_zero_pos_int() {
    let number = Number { n: N::PosInt(0) };
    assert_eq!(number.as_f32(), Some(0.0));
}

#[test]
fn test_as_f32_zero_neg_int() {
    let number = Number { n: N::NegInt(0) };
    assert_eq!(number.as_f32(), Some(0.0));
}

