// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u32),
    NegInt(i32),
    Float(f64),
}

#[derive(Debug)]
enum Unexpected {
    Unsigned(u32),
    Signed(i32),
    Float(f64),
}

struct TestStruct {
    n: N,
}

impl TestStruct {
    pub(crate) fn unexpected(&self) -> Unexpected {
        match self.n {
            N::PosInt(u) => Unexpected::Unsigned(u),
            N::NegInt(i) => Unexpected::Signed(i),
            N::Float(f) => Unexpected::Float(f),
        }
    }
}

#[test]
fn test_unexpected_with_positive_integer() {
    let test_value = TestStruct { n: N::PosInt(42) };
    match test_value.unexpected() {
        Unexpected::Unsigned(u) => assert_eq!(u, 42),
        _ => panic!("Expected Unexpected::Unsigned variant with value 42"),
    }
}

#[test]
fn test_unexpected_with_large_positive_integer() {
    let test_value = TestStruct { n: N::PosInt(100_000) };
    match test_value.unexpected() {
        Unexpected::Unsigned(u) => assert_eq!(u, 100_000),
        _ => panic!("Expected Unexpected::Unsigned variant with value 100_000"),
    }
}

