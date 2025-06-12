// Answer 0

#[test]
fn test_parse_object_colon_valid_case() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[b' ', b' ', b':']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_object_colon();
}

#[test]
fn test_parse_object_colon_unexpected_char() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[b' ', b' ', b',']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_object_colon();
}

#[test]
fn test_parse_object_colon_eof() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[b' ', b' ']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_object_colon();
}

#[test]
fn test_parse_object_colon_multiple_whitespace() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[b' ', b'\n', b'\t', b' ', b':']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_object_colon();
}

#[test]
fn test_parse_object_colon_whitespace_only() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[b' ', b' ', b' ', b' ']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_object_colon();
}

#[test]
#[should_panic]
fn test_parse_object_colon_not_enough_input() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[b' ', b' ', b' ']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_object_colon();
}

#[test]
fn test_parse_object_colon_edge_case() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[b':']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_object_colon();
}

