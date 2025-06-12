// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

struct MyStruct {
    n: N,
}

impl MyStruct {
    pub(crate) fn unexpected(&self) -> Unexpected {
        match self.n {
            N::PosInt(u) => Unexpected::Unsigned(u),
            N::NegInt(i) => Unexpected::Signed(i),
            N::Float(f) => Unexpected::Float(f),
        }
    }
}

#[derive(Debug)]
enum Unexpected {
    Unsigned(u64),
    Signed(i64),
    Float(f64),
}

#[test]
fn test_unexpected_float() {
    let my_value = MyStruct { n: N::Float(3.14) };
    match my_value.unexpected() {
        Unexpected::Float(f) => assert_eq!(f, 3.14),
        _ => panic!("Expected Unexpected::Float(3.14)"),
    }
}

#[test]
fn test_unexpected_float_negative() {
    let my_value = MyStruct { n: N::Float(-2.71) };
    match my_value.unexpected() {
        Unexpected::Float(f) => assert_eq!(f, -2.71),
        _ => panic!("Expected Unexpected::Float(-2.71)"),
    }
}

