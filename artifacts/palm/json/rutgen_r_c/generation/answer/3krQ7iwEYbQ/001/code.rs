// Answer 0

#[test]
fn test_deserialize_ignored_any_with_valid_visitor() {
    struct ValidVisitor;

    impl<'de> Visitor<'de> for ValidVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        forward_to_deserialize_any! {
            bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, char,
            str, string, bytes, byte_buf, option, seq, tuple, tuple_struct,
            map, struct, enum, identifier, ignored_any
        }
    }

    let value: Value = Value::Null; // The actual value doesn't matter due to drop(self);
    let result = value.deserialize_ignored_any(ValidVisitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_ignored_any_with_panic_condition() {
    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            panic!("This visitor should not be invoked");
        }

        forward_to_deserialize_any! {
            bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, char,
            str, string, bytes, byte_buf, option, seq, tuple, tuple_struct,
            map, struct, enum, identifier, ignored_any
        }
    }

    let value: Value = Value::Null;
    let _ = value.deserialize_ignored_any(PanicVisitor);
}

