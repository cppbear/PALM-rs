// Answer 0

#[test]
fn test_parse_integer_leading_zero() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"0"),
        scratch: vec![],
        remaining_depth: 10,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_single_digit() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"5"),
        scratch: vec![],
        remaining_depth: 10,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_two_digit() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"42"),
        scratch: vec![],
        remaining_depth: 10,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_overflow() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"18446744073709551615"), // u64::MAX as string
        scratch: vec![],
        remaining_depth: 10,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_invalid_zero_lead() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"00"),
        scratch: vec![],
        remaining_depth: 10,
    };
    let result = deserializer.parse_integer(true);
    assert!(result.is_err());
}

#[test]
fn test_parse_integer_invalid_character() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"a5"),
        scratch: vec![],
        remaining_depth: 10,
    };
    let result = deserializer.parse_integer(true);
    assert!(result.is_err());
}

#[test]
fn test_parse_integer_negative() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"-5"),
        scratch: vec![],
        remaining_depth: 10,
    };
    let _ = deserializer.parse_integer(false);
}

