// Answer 0

#[test]
fn test_parse_decimal_valid_case_1() {
    let mut deserializer = Deserializer {
        read: StrRead::new("123.456e2"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    let result = deserializer.parse_decimal(true, 123, 0);
}

#[test]
fn test_parse_decimal_valid_case_2() {
    let mut deserializer = Deserializer {
        read: StrRead::new("78.90e-1"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    let result = deserializer.parse_decimal(true, 78, 0);
}

#[test]
fn test_parse_decimal_overflow_case() {
    let mut deserializer = Deserializer {
        read: StrRead::new("1844674407370955162.0"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    let result = deserializer.parse_decimal(true, 1844674407370955161, 0);
}

#[test]
fn test_parse_decimal_valid_large_exponent() {
    let mut deserializer = Deserializer {
        read: StrRead::new("1.0001e308"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    let result = deserializer.parse_decimal(true, 1, 0);
}

#[test]
fn test_parse_decimal_no_digits_after_decimal() {
    let mut deserializer = Deserializer {
        read: StrRead::new("123."),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    let result = deserializer.parse_decimal(true, 123, 0);
}

#[test]
fn test_parse_decimal_zero_case() {
    let mut deserializer = Deserializer {
        read: StrRead::new("0.001"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    let result = deserializer.parse_decimal(true, 0, 0);
}

