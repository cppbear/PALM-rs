// Answer 0

#[test]
fn test_deserialize_bool_true_valid_whitespace() {
    struct Visitor;
    impl de::Visitor<'de> for Visitor {
        type Value = bool;
        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let input = b"  true";
    let deserializer = Deserializer {
        read: StrRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    deserializer.deserialize_bool(Visitor);
}

#[test]
fn test_deserialize_bool_false_valid_whitespace() {
    struct Visitor;
    impl de::Visitor<'de> for Visitor {
        type Value = bool;
        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let input = b"\tfalse";
    let deserializer = Deserializer {
        read: StrRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    deserializer.deserialize_bool(Visitor);
}

#[test]
fn test_deserialize_bool_invalid_character() {
    struct Visitor;
    impl de::Visitor<'de> for Visitor {
        type Value = bool;
        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            Err(Error)
        }
    }

    let input = b"unknown";
    let deserializer = Deserializer {
        read: StrRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    deserializer.deserialize_bool(Visitor);
}

#[test]
fn test_deserialize_bool_error_on_whitespace() {
    struct Visitor;
    impl de::Visitor<'de> for Visitor {
        type Value = bool;
        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            Err(Error)
        }
    }

    let input = b" \n \t ";
    let deserializer = Deserializer {
        read: StrRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    deserializer.deserialize_bool(Visitor);
}

#[test]
#[should_panic]
fn test_deserialize_bool_error_invalid_type() {
    struct Visitor;
    impl de::Visitor<'de> for Visitor {
        type Value = bool;
        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            Err(Error)
        }
    }

    let input = b"12";
    let deserializer = Deserializer {
        read: StrRead::new(input),
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    deserializer.deserialize_bool(Visitor);
}

