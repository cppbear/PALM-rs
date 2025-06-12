// Answer 0

#[test]
fn test_visit_array_ref_success() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct MockVisitor {
        expected_count: usize,
        visited_count: usize,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Vec<Value>;

        fn visit_seq<S>(self, _: &mut S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            self.visited_count += 1;
            Ok(vec![Value::Number(1.into()), Value::Number(2.into())])
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of numbers")
        }
    }

    let array = &[Value::Number(1.into()), Value::Number(2.into())];
    let visitor = MockVisitor { expected_count: 2, visited_count: 0 };

    let result = visit_array_ref(array, visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![Value::Number(1.into()), Value::Number(2.into())]);
}

#[test]
#[should_panic]
fn test_visit_array_ref_failure_remaining_elements() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = Vec<Value>;

        fn visit_seq<S>(self, _: &mut S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Err(de::Error::custom("force panic"))
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an empty sequence")
        }
    }

    let array = &[Value::Number(1.into()), Value::Number(2.into())];
    let visitor = PanicVisitor;

    let _ = visit_array_ref(array, visitor);
}

#[test]
fn test_visit_array_ref_insufficient_elements() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct InsufficientVisitor;

    impl<'de> Visitor<'de> for InsufficientVisitor {
        type Value = Vec<Value>;

        fn visit_seq<S>(self, _: &mut S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Ok(vec![Value::Number(1.into())])
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array with at least two elements")
        }
    }

    let array = &[Value::Number(1.into()), Value::Number(2.into())];
    let visitor = InsufficientVisitor;

    let result = visit_array_ref(array, visitor);
    assert!(result.is_err());
}

