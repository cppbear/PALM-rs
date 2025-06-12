// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u32),
    NegInt(i32),
    Float(f32),
}

#[derive(Debug)]
enum Unexpected {
    Unsigned(u32),
    Signed(i32),
    Float(f32),
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
fn test_unexpected_pos_int() {
    let test_struct = TestStruct { n: N::PosInt(42) };
    match test_struct.unexpected() {
        Unexpected::Unsigned(value) => assert_eq!(value, 42),
        _ => panic!("Expected an Unsigned variant"),
    }
}

#[test]
fn test_unexpected_neg_int() {
    let test_struct = TestStruct { n: N::NegInt(-42) };
    match test_struct.unexpected() {
        Unexpected::Signed(value) => assert_eq!(value, -42),
        _ => panic!("Expected a Signed variant"),
    }
}

#[test]
fn test_unexpected_float() {
    let test_struct = TestStruct { n: N::Float(3.14) };
    match test_struct.unexpected() {
        Unexpected::Float(value) => assert_eq!(value, 3.14),
        _ => panic!("Expected a Float variant"),
    }
}

