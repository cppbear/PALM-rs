// Answer 0

#[test]
fn test_parse_decimal_invalid_number_without_digits_after_decimal() {
    let read = MockRead::with_bytes(b"123.").unwrap();
    let mut deserializer = Deserializer::new(read);

    let result = deserializer.parse_decimal(true, 3, 0);
}

#[test]
fn test_parse_decimal_eof_after_decimal() {
    let read = MockRead::with_bytes(b"123.").unwrap();
    let mut deserializer = Deserializer::new(read);

    let result = deserializer.parse_decimal(true, 123, 0);
}

#[test]
fn test_parse_decimal_invalid_number_no_eof() {
    let read = MockRead::with_bytes(b"123.0e").unwrap();
    let mut deserializer = Deserializer::new(read);

    let result = deserializer.parse_decimal(true, 123, 0);
}

#[test]
fn test_parse_decimal_overflow_scenario() {
    let read = MockRead::with_bytes(b"1234567890123456789012345678901234567890.0").unwrap();
    let mut deserializer = Deserializer::new(read);

    let result = deserializer.parse_decimal(true, u64::MAX, 0);
}

