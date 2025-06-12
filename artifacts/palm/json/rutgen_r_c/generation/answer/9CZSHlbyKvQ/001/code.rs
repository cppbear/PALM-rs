// Answer 0

#[test]
fn test_deserialize_bool_invalid_type() {
    use serde::de::Visitor;
    use crate::value::Value;
    use crate::error::Error;

    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_bool(self, _value: bool) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_null(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        // Implement other required methods as needed, but for this test we can ignore them.
        forward_to_deserialize_any! {
            i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, char, str,
            string, bytes, byte_buf, unit, unit_struct, newtype_struct, seq,
            tuple, tuple_struct, map, struct, identifier, ignored_any
        }
    }

    let invalid_value = Value::String("invalid".to_string()); // This will not match Value::Bool
    let visitor = MockVisitor;

    // Call the function under test
    let result = invalid_value.deserialize_bool(visitor);

    // Verify the expected Err variant is returned
    assert!(result.is_err());
}

#[test]
fn test_deserialize_bool_not_a_bool() {
    use serde::de::Visitor;
    use crate::value::Value;
    use crate::error::Error;

    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_bool(self, _value: bool) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_null(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        // Implement other required methods as needed, but for this test we can ignore them.
        forward_to_deserialize_any! {
            i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, char, str,
            string, bytes, byte_buf, unit, unit_struct, newtype_struct, seq,
            tuple, tuple_struct, map, struct, identifier, ignored_any
        }
    }

    let invalid_value = Value::Array(vec![Value::Null]); // This will not match Value::Bool
    let visitor = MockVisitor;

    // Call the function under test
    let result = invalid_value.deserialize_bool(visitor);

    // Verify the expected Err variant is returned
    assert!(result.is_err());
}

