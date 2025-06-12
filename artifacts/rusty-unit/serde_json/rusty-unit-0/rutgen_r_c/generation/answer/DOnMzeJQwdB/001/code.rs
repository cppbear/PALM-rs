// Answer 0

#[test]
fn test_deserialize_bool_invalid_type() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            Ok(())
        }

        // Implement other required methods for the Visitor trait.
        fn visit_i64(self, _: i64) -> Result<Self::Value, Error> {
            Err(Error) // Just to fulfill the trait requirement
        }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Error> {
            Err(Error) // Just to fulfill the trait requirement
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let visitor = TestVisitor;

    // Constraint: self matches _ is true, so we use a different Value variant here
    let value = Value::Null;

    // We expect an error since the value is not of type Bool
    let result = value.deserialize_bool(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_bool_invalid_type_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            Ok(())
        }

        // Implement other required methods for the Visitor trait.
        fn visit_i64(self, _: i64) -> Result<Self::Value, Error> {
            Err(Error) // Just to fulfill the trait requirement
        }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Error> {
            Err(Error) // Just to fulfill the trait requirement
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let visitor = TestVisitor;

    // Constraint: self matches _ is true, so we use a different Value variant here
    let value = Value::String(String::from("not a bool"));

    // We expect an error since the value is not of type Bool
    let result = value.deserialize_bool(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_bool_invalid_type_array() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            Ok(())
        }

        // Implement other required methods for the Visitor trait.
        fn visit_i64(self, _: i64) -> Result<Self::Value, Error> {
            Err(Error) // Just to fulfill the trait requirement
        }
        fn visit_u64(self, _: u64) -> Result<Self::Value, Error> {
            Err(Error) // Just to fulfill the trait requirement
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let visitor = TestVisitor;

    // Constraint: self matches _ is true, so we use a different Value variant here
    let value = Value::Array(vec![Value::Null]);

    // We expect an error since the value is not of type Bool
    let result = value.deserialize_bool(visitor);
    assert!(result.is_err());
}

