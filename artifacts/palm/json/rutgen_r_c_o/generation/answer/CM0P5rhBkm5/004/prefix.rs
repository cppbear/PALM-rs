// Answer 0

#[test]
fn test_deserialize_option_some() {
    let mut deserializer = Deserializer {
        read: StrRead::new("5"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(visitor);
}

#[test]
fn test_deserialize_option_none() {
    let mut deserializer = Deserializer {
        read: StrRead::new("null"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_option_invalid_ident() {
    let mut deserializer = Deserializer {
        read: StrRead::new("nul"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_option_invalid_input() {
    let mut deserializer = Deserializer {
        read: StrRead::new("nonsensical"),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(visitor);
}

#[test]
fn test_deserialize_option_empty_string() {
    let mut deserializer = Deserializer {
        read: StrRead::new(""),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.deserialize_option(visitor);
}

