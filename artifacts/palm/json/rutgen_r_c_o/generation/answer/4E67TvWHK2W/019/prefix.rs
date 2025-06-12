// Answer 0

#[test]
fn test_parse_decimal_edge_case_positive() {
    let mut deserializer = Deserializer {
        read: MyReader::new(vec![b'1', b'0', b'e', b'1']),
        scratch: Vec::new(),
        remaining_depth: 10,
    };
    let _ = deserializer.parse_decimal(true, 99999999999999999999, 308);
}

#[test]
fn test_parse_decimal_edge_case_negative() {
    let mut deserializer = Deserializer {
        read: MyReader::new(vec![b'1', b'0', b'E', b'1']),
        scratch: Vec::new(),
        remaining_depth: 10,
    };
    let _ = deserializer.parse_decimal(true, 99999999999999999999, 308);
}

#[test]
fn test_parse_decimal_invalid_after_point() {
    let mut deserializer = Deserializer {
        read: MyReader::new(vec![b'1', b'E', b'1']),
        scratch: Vec::new(),
        remaining_depth: 10,
    };
    let _ = deserializer.parse_decimal(true, 99999999999999999999, 0);
}

#[test]
fn test_parse_decimal_zero_after_point() {
    let mut deserializer = Deserializer {
        read: MyReader::new(vec![b'1', b'0', b'0', b'0']),
        scratch: Vec::new(),
        remaining_depth: 10,
    };
    let _ = deserializer.parse_decimal(true, 99999999999999999999, 1);
}

