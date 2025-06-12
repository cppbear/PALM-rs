// Answer 0

#[test]
fn test_fmt_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let pos_number = Number { n: N::PosInt(42) };
    let mut buffer = std::fmt::Formatter::new();

    let result = pos_number.fmt(&mut buffer);
    assert_eq!(result, Ok(()));
    assert_eq!(buffer.to_string(), "42");
}

#[test]
fn test_fmt_large_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let large_pos_number = Number { n: N::PosInt(u64::MAX) };
    let mut buffer = std::fmt::Formatter::new();

    let result = large_pos_number.fmt(&mut buffer);
    assert_eq!(result, Ok(()));
    assert_eq!(buffer.to_string(), "18446744073709551615");
}

#[test]
fn test_fmt_small_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let small_pos_number = Number { n: N::PosInt(0) };
    let mut buffer = std::fmt::Formatter::new();

    let result = small_pos_number.fmt(&mut buffer);
    assert_eq!(result, Ok(()));
    assert_eq!(buffer.to_string(), "0");
}

