// Answer 0

#[test]
fn test_ignore_decimal_with_valid_number() {
    let input = b"123e";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    deserializer.ignore_decimal();
}

#[test]
fn test_ignore_decimal_with_valid_number_and_exponent() {
    let input = b"456E789";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    deserializer.ignore_decimal();
}

#[test]
fn test_ignore_decimal_with_only_digits() {
    let input = b"789";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    deserializer.ignore_decimal();
}

#[test]
fn test_ignore_decimal_with_multiple_digits_and_exponent() {
    let input = b"0.12345e";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    deserializer.ignore_decimal();
}

#[test]
fn test_ignore_decimal_with_no_exponent() {
    let input = b"234567";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    deserializer.ignore_decimal();
}

#[should_panic]
fn test_ignore_decimal_with_no_digits() {
    let input = b"";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    deserializer.ignore_decimal();
}

#[should_panic]
fn test_ignore_decimal_with_invalid_character() {
    let input = b"abc";
    let mut deserializer = Deserializer {
        read: SliceRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    deserializer.ignore_decimal();
}

