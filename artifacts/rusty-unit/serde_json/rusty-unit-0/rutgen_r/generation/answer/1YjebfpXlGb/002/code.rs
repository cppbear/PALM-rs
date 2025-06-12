// Answer 0

#[derive(Debug)]
enum N {
    PosInt(i32),
    NegInt(i32),
    Float(f32),
}

struct TestStruct {
    n: N,
}

impl TestStruct {
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
fn test_as_f32_neg_int() {
    let test_instance = TestStruct { n: N::NegInt(-42) };
    assert_eq!(test_instance.as_f32(), Some(-42.0));
}

#[test]
fn test_as_f32_pos_int() {
    let test_instance = TestStruct { n: N::PosInt(42) };
    assert_eq!(test_instance.as_f32(), Some(42.0));
}

#[test]
fn test_as_f32_float() {
    let test_instance = TestStruct { n: N::Float(3.14) };
    assert_eq!(test_instance.as_f32(), Some(3.14));
}

