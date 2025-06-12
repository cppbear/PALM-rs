// Answer 0

#[test]
fn test_deserialize_option_some() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"some_value"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(/* visitor */);
}

#[test]
fn test_deserialize_option_none() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"null"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(/* visitor */);
}

#[test]
#[should_panic]
fn test_deserialize_option_err() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b""),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(/* visitor */);
}

#[test]
fn test_deserialize_option_whitespace() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"   null"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(/* visitor */);
}

#[test]
fn test_deserialize_option_invalid() {
    let mut deserializer = Deserializer {
        read: StrRead::new(b"not_a_valid_option"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(/* visitor */);
}

