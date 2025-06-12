// Answer 0

#[test]
fn test_parse_integer_zero() {
    let read = vec![b'0'];
    let mut deserializer = Deserializer { read: SliceRead::new(&read), scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_valid_single_digit() {
    let read = vec![b'5'];
    let mut deserializer = Deserializer { read: SliceRead::new(&read), scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_valid_double_digit() {
    let read = vec![b'1', b'2'];
    let mut deserializer = Deserializer { read: SliceRead::new(&read), scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_valid_triple_digit() {
    let read = vec![b'1', b'0', b'0'];
    let mut deserializer = Deserializer { read: SliceRead::new(&read), scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_valid_four_digit() {
    let read = vec![b'2', b'5', b'3', b'1'];
    let mut deserializer = Deserializer { read: SliceRead::new(&read), scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_overflow() {
    let read = vec![b'1', b'8', b'4', b'4', b'6', b'7', b'4', b'0', b'7', b'3', b'7', b'0', b'9', b'5', b'5', b'1', b'6', b'1', b'5'];
    let mut deserializer = Deserializer { read: SliceRead::new(&read), scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_leading_zero() {
    let read = vec![b'0', b'0'];
    let mut deserializer = Deserializer { read: SliceRead::new(&read), scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_invalid_number() {
    let read = vec![b'0', b'1'];
    let mut deserializer = Deserializer { read: SliceRead::new(&read), scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_missing_value() {
    let read = vec![];
    let mut deserializer = Deserializer { read: SliceRead::new(&read), scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_integer(true);
}

