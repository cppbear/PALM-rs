// Answer 0

#[test]
fn test_peek_invalid_type_with_null() {
    let mut deserializer = Deserializer {
        read: StrRead::new("null"),
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let expected: &dyn Expected = &Unexpected::Unit;
    deserializer.peek_invalid_type(expected);
}

#[test]
fn test_peek_invalid_type_with_true() {
    let mut deserializer = Deserializer {
        read: StrRead::new("true"),
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let expected: &dyn Expected = &Unexpected::Bool(true);
    deserializer.peek_invalid_type(expected);
}

#[test]
fn test_peek_invalid_type_with_false() {
    let mut deserializer = Deserializer {
        read: StrRead::new("false"),
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let expected: &dyn Expected = &Unexpected::Bool(false);
    deserializer.peek_invalid_type(expected);
}

#[test]
fn test_peek_invalid_type_with_string() {
    let mut deserializer = Deserializer {
        read: StrRead::new("\"string\""),
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let expected: &dyn Expected = &Unexpected::Str("string");
    deserializer.peek_invalid_type(expected);
}

#[test]
fn test_peek_invalid_type_with_negative_number() {
    let mut deserializer = Deserializer {
        read: StrRead::new("-42"),
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let expected: &dyn Expected = &Unexpected::Other("number");
    deserializer.peek_invalid_type(expected);
}

#[test]
fn test_peek_invalid_type_with_positive_number() {
    let mut deserializer = Deserializer {
        read: StrRead::new("42"),
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let expected: &dyn Expected = &Unexpected::Other("number");
    deserializer.peek_invalid_type(expected);
}

#[test]
fn test_peek_invalid_type_with_array() {
    let mut deserializer = Deserializer {
        read: StrRead::new("[1, 2, 3]"),
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let expected: &dyn Expected = &Unexpected::Seq;
    deserializer.peek_invalid_type(expected);
}

#[test]
fn test_peek_invalid_type_with_object() {
    let mut deserializer = Deserializer {
        read: StrRead::new("{\"key\": \"value\"}"),
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let expected: &dyn Expected = &Unexpected::Map;
    deserializer.peek_invalid_type(expected);
}

