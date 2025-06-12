// Answer 0

#[test]
fn test_as_u64_float_positive() {
    let number = Number {
        n: N::Float(1.5),
    };
    number.as_u64();
}

#[test]
fn test_as_u64_float_negative() {
    let number = Number {
        n: N::Float(-3.14),
    };
    number.as_u64();
}

#[test]
fn test_as_u64_float_zero() {
    let number = Number {
        n: N::Float(0.0),
    };
    number.as_u64();
}

#[test]
fn test_as_u64_float_large() {
    let number = Number {
        n: N::Float(1e10),
    };
    number.as_u64();
}

#[test]
fn test_as_u64_float_small() {
    let number = Number {
        n: N::Float(1e-10),
    };
    number.as_u64();
}

