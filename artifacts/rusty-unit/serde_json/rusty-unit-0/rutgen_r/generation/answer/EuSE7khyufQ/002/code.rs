// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

#[derive(Debug)]
enum Unexpected {
    Unsigned(u64),
    Signed(i64),
    Float(f64),
}

struct Number {
    n: N,
}

impl Number {
    pub(crate) fn unexpected(&self) -> Unexpected {
        match self.n {
            N::PosInt(u) => Unexpected::Unsigned(u),
            N::NegInt(i) => Unexpected::Signed(i),
            N::Float(f) => Unexpected::Float(f),
        }
    }
}

#[test]
fn test_unexpected_negint() {
    let number = Number { n: N::NegInt(-42) };
    match number.unexpected() {
        Unexpected::Signed(value) => assert_eq!(value, -42),
        _ => panic!("Expected a Signed variant"),
    }
}

#[test]
fn test_unexpected_negint_zero() {
    let number = Number { n: N::NegInt(0) };
    match number.unexpected() {
        Unexpected::Signed(value) => assert_eq!(value, 0),
        _ => panic!("Expected a Signed variant"),
    }
}

