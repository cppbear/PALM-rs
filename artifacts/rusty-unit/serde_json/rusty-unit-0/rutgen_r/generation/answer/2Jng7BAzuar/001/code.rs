// Answer 0

#[test]
fn test_tuple_variant_with_empty_array() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_array<E>(self, _vals: &[Value]) -> Result<Self::Value, E> {
            // Just for the sake of the test, will not visit an array.
            Ok(())
        }
    }

    let input = Value::Array(vec![]);
    let result = tuple_variant(Some(input), 0, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_tuple_variant_with_non_empty_array() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_array<E>(self, _vals: &[Value]) -> Result<Self::Value, E> {
            // Just for the sake of the test, will not visit an array.
            Ok(())
        }
    }

    let input = Value::Array(vec![Value::from(1), Value::from(2)]);
    let result = tuple_variant(Some(input), 2, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_tuple_variant_with_other_value() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_array<E>(self, _vals: &[Value]) -> Result<Self::Value, E> {
            // Just for the sake of the test, will not visit an array.
            Ok(())
        }
    }

    let input = Value::String("not an array".to_string());
    let result = tuple_variant(Some(input), 1, TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_tuple_variant_with_none() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_array<E>(self, _vals: &[Value]) -> Result<Self::Value, E> {
            // Just for the sake of the test, will not visit an array.
            Ok(())
        }
    }

    let input: Option<Value> = None;
    let result = tuple_variant(input, 0, TestVisitor);
    assert!(result.is_err());
}

