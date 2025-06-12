// Answer 0

#[test]
fn test_deserialize_any_borrowed() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Other required visitor methods can be left unimplemented for this test
        forward_to_deserialize_any! {
            i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
            bool unit unit_struct newtype_struct seq map
            enum identifier string bytes byte_buf option some
            none tuple tuple_struct
        }
    }

    let deserializer = BorrowedCowStrDeserializer {
        value: Cow::Borrowed("test"),
    };

    let result = deserializer.deserialize_any(TestVisitor);
    assert_eq!(result.unwrap(), "test");
}

#[test]
#[cfg(feature = "std")]
fn test_deserialize_any_owned() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Other required visitor methods can be left unimplemented for this test
        forward_to_deserialize_any! {
            i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
            bool unit unit_struct newtype_struct seq map
            enum identifier string bytes byte_buf option some
            none tuple tuple_struct
        }
    }

    let deserializer = BorrowedCowStrDeserializer {
        value: Cow::Owned("owned_test".to_string()),
    };

    let result = deserializer.deserialize_any(TestVisitor);
    assert_eq!(result.unwrap(), "owned_test");
}

