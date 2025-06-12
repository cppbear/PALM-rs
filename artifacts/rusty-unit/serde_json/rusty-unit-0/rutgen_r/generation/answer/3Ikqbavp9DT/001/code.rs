// Answer 0

#[test]
fn test_fmt_float_positive() {
    struct TestNumber {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    let test_value = TestNumber { n: N::Float(3.14) };
    let mut buffer = std::fmt::Formatter::new();
    let result = test_value.fmt(&mut buffer);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_fmt_float_negative() {
    struct TestNumber {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    let test_value = TestNumber { n: N::Float(-2.71) };
    let mut buffer = std::fmt::Formatter::new();
    let result = test_value.fmt(&mut buffer);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_fmt_float_zero() {
    struct TestNumber {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    let test_value = TestNumber { n: N::Float(0.0) };
    let mut buffer = std::fmt::Formatter::new();
    let result = test_value.fmt(&mut buffer);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_fmt_float_infinity() {
    struct TestNumber {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    let test_value = TestNumber { n: N::Float(f64::INFINITY) };
    let mut buffer = std::fmt::Formatter::new();
    let result = test_value.fmt(&mut buffer);
    assert_eq!(result, Ok(()));
} 

#[test]
fn test_fmt_float_nan() {
    struct TestNumber {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    let test_value = TestNumber { n: N::Float(f64::NAN) };
    let mut buffer = std::fmt::Formatter::new();
    let result = test_value.fmt(&mut buffer);
    assert_eq!(result, Ok(()));
}

