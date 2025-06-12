// Answer 0

#[test]
fn test_ignore_decimal_with_valid_number() {
    let input = vec![b'1', b'2', b'3', b'.', b'4', b'5'];
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    let _ = deserializer.ignore_decimal();
}

#[test]
fn test_ignore_decimal_with_no_digits() {
    let input = vec![b'.', b'4', b'5'];
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    let result = deserializer.ignore_decimal();
    let _ = result.unwrap_err(); // expects an error
}

#[test]
fn test_ignore_decimal_with_exponent() {
    let input = vec![b'3', b'4', b'e', b'2'];
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    let _ = deserializer.ignore_decimal();
}

#[test]
fn test_ignore_decimal_with_invalid_character() {
    let input = vec![b'0', b'0', b'0', b'x'];
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    let result = deserializer.ignore_decimal();
    let _ = result.unwrap_err(); // expects an error
}

#[test]
fn test_ignore_decimal_with_empty_sequence() {
    let input: Vec<u8> = vec![];
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    let result = deserializer.ignore_decimal();
    let _ = result.unwrap_err(); // expects an error
}

#[test]
fn test_ignore_decimal_with_non_digit_character() {
    let input = vec![b'a'];
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    let result = deserializer.ignore_decimal();
    let _ = result.unwrap_err(); // expects an error
}

