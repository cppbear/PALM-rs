// Answer 0

#[test]
fn test_string_deserializer_deserialize_any() {
    use crate::de::Visitor;
    use crate::de::{self, Error};

    struct MockVisitor {
        expected: String,
        called: bool,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            assert_eq!(value, self.expected);
            Ok(value)
        }

        // Other visit methods are not needed for this test
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            unimplemented!()
        }
    }

    let expected_value = String::from("test string");
    let deserializer = StringDeserializer {
        value: expected_value.clone(),
        marker: PhantomData,
    };
    let visitor = MockVisitor {
        expected: expected_value,
        called: false,
    };

    let result: Result<String, Box<str>> = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}

#[test]
fn test_string_deserializer_deserialize_any_error() {
    use crate::de::Visitor;
    use crate::de::{self, Error};

    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_string<E>(self, _: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("Visit error"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            unimplemented!()
        }
    }

    let deserializer = StringDeserializer {
        value: String::from("test string"),
        marker: PhantomData,
    };
    let visitor = MockVisitor;

    let result: Result<String, Box<str>> = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
}

