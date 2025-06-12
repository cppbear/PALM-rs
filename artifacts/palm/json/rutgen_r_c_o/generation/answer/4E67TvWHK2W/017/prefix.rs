// Answer 0

#[test]
fn test_parse_decimal_panic_overflow() {
    let input = b"1.234e10";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 10,
    };
    let _ = deserializer.parse_decimal(true, u64::MAX, 0);
}

#[test]
fn test_parse_decimal_valid_case() {
    let input = b"123.456e2";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 10,
    };
    let _ = deserializer.parse_decimal(true, 123, 0);
}

#[test]
fn test_parse_decimal_no_digits_after_point() {
    let input = b"123.";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 10,
    };
    let _ = deserializer.parse_decimal(true, 123, 0);
}

#[test]
fn test_parse_decimal_with_negative_exponent() {
    let input = b"1.23e-3";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 10,
    };
    let _ = deserializer.parse_decimal(true, 123, 0);
}

#[test]
fn test_parse_decimal_zero_significand() {
    let input = b"0.123e2";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 10,
    };
    let _ = deserializer.parse_decimal(true, 0, 0);
}

