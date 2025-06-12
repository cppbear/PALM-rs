// Answer 0

#[test]
fn test_serialize_i16() {
    struct Serializer;

    impl Serializer {
        fn serialize_i16(self, v: i16) -> Result<Content, &'static str> {
            Ok(Content::I16(v))
        }
    }

    enum Content {
        I16(i16),
    }

    let serializer = Serializer;
    let result = serializer.serialize_i16(42);
    assert!(result.is_ok());
    if let Ok(Content::I16(value)) = result {
        assert_eq!(value, 42);
    } else {
        panic!("Expected Ok(Content::I16), but got a different result");
    }
}

#[test]
fn test_serialize_i16_negative_value() {
    struct Serializer;

    impl Serializer {
        fn serialize_i16(self, v: i16) -> Result<Content, &'static str> {
            Ok(Content::I16(v))
        }
    }

    enum Content {
        I16(i16),
    }

    let serializer = Serializer;
    let result = serializer.serialize_i16(-42);
    assert!(result.is_ok());
    if let Ok(Content::I16(value)) = result {
        assert_eq!(value, -42);
    } else {
        panic!("Expected Ok(Content::I16), but got a different result");
    }
}

#[test]
fn test_serialize_i16_zero() {
    struct Serializer;

    impl Serializer {
        fn serialize_i16(self, v: i16) -> Result<Content, &'static str> {
            Ok(Content::I16(v))
        }
    }

    enum Content {
        I16(i16),
    }

    let serializer = Serializer;
    let result = serializer.serialize_i16(0);
    assert!(result.is_ok());
    if let Ok(Content::I16(value)) = result {
        assert_eq!(value, 0);
    } else {
        panic!("Expected Ok(Content::I16), but got a different result");
    }
}

