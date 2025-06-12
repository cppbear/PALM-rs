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
fn test_unexpected_with_negative_integer() {
    let number = Number { n: N::NegInt(-42) };
    if let Unexpected::Signed(value) = number.unexpected() {
        assert_eq!(value, -42);
    } else {
        panic!("Expected Unexpected::Signed variant");
    }
}

#[test]
fn test_unexpected_with_zero_negative_integer() {
    let number = Number { n: N::NegInt(0) };
    if let Unexpected::Signed(value) = number.unexpected() {
        assert_eq!(value, 0);
    } else {
        panic!("Expected Unexpected::Signed variant");
    }
}

