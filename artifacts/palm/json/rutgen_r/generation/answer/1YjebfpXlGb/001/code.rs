// Answer 0

#[derive(Debug)]
enum N {
    PosInt(i64),
    NegInt(i64),
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
fn test_as_f32_positive_float() {
    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.as_f32(), Some(3.14f32));
}

#[test]
fn test_as_f32_negative_float() {
    let number = Number { n: N::Float(-1.5) };
    assert_eq!(number.as_f32(), Some(-1.5f32));
}

#[test]
fn test_as_f32_zero_float() {
    let number = Number { n: N::Float(0.0) };
    assert_eq!(number.as_f32(), Some(0.0f32));
}

#[test]
fn test_as_f32_large_float() {
    let number = Number { n: N::Float(1e10) };
    assert_eq!(number.as_f32(), Some(1e10f32));
}

#[test]
fn test_as_f32_small_float() {
    let number = Number { n: N::Float(1e-10) };
    assert_eq!(number.as_f32(), Some(1e-10f32));
}

