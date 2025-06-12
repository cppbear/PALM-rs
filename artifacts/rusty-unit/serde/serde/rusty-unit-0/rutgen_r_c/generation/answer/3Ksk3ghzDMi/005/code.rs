// Answer 0

#[test]
fn test_deserialize_integer_i8() {
    use crate::de::{Visitor, Error};

    struct TestVisitor {
        value: Option<i8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i8;

        fn visit_i8(self, value: i8) -> Result<i8, Error> {
            Ok(value)
        }

        fn visit_u8(self, _: u8) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got u8"))
        }

        fn visit_i16(self, _: i16) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got i16"))
        }

        fn visit_i32(self, _: i32) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got i32"))
        }

        fn visit_i64(self, _: i64) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got i64"))
        }

        fn visit_u16(self, _: u16) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got u16"))
        }

        fn visit_u32(self, _: u32) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got u32"))
        }

        fn visit_u64(self, _: u64) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got u64"))
        }

        fn visit_f32(self, _: f32) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got f32"))
        }

        fn visit_f64(self, _: f64) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got f64"))
        }

        fn visit_char(self, _: char) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got char"))
        }

        fn visit_bytes(self, _: &[u8]) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got bytes"))
        }

        fn visit_string(self, _: &str) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got string"))
        }

        fn visit_unit(self) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got unit"))
        }

        fn visit_seq<V>(self, _: V) -> Result<i8, Error>
        where
            V: crate::de::SeqAccess<'de>,
        {
            Err(Error::custom("Expected i8, got sequence"))
        }

        fn visit_map<V>(self, _: V) -> Result<i8, Error>
        where
            V: crate::de::MapAccess<'de>,
        {
            Err(Error::custom("Expected i8, got map"))
        }

        fn visit_option<V>(self, _: V) -> Result<i8, Error>
        where
            V: crate::de::OptionAccess<'de>,
        {
            Err(Error::custom("Expected i8, got option"))
        }
    }

    let content = Content::I8(42);
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData };
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_integer(visitor);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_deserialize_integer_invalid_type() {
    use crate::de::{Visitor, Error};

    struct TestVisitor {
        value: Option<i8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i8;

        fn visit_i8(self, value: i8) -> Result<i8, Error> {
            Ok(value)
        }

        fn visit_u8(self, _: u8) -> Result<i8, Error> {
            Err(Error::custom("Expected i8, got u8"))
        }

        // Other visit methods return similar errors for different types ...
        // They can be defined similar to the previous test above.
    }

    let content = Content::F32(42.0); // Invalid type for i8
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData };
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_integer(visitor);
    assert!(result.is_err());
}

