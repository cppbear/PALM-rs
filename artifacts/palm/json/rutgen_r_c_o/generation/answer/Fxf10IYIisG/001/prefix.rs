// Answer 0

#[test]
fn test_deserialize_ignored_any_with_valid_visitor() {
    struct ValidVisitor;

    impl<'de> Visitor<'de> for ValidVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
        forward_to_deserialize_any!();
    }

    let value = Value::Null;
    let _ = value.deserialize_ignored_any(ValidVisitor);
}

#[test]
#[should_panic]
fn test_deserialize_ignored_any_with_invalid_visitor() {
    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value, Error> {
            panic!("This visitor should panic");
        }
        forward_to_deserialize_any!();
    }

    let value = Value::Null;
    let _ = value.deserialize_ignored_any(InvalidVisitor);
}

