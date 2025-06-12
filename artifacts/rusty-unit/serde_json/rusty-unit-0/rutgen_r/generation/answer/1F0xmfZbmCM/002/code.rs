// Answer 0

#[test]
fn test_visit_array_ref_success() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error, SeqRefDeserializer};

    struct TestVisitor {
        expected: Vec<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<S>(self, _seq: &mut S) -> Result<Self::Value, Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Ok(self.expected.clone())
        }
    }

    let array = vec![Value::from(1), Value::from(2)];
    let visitor = TestVisitor {
        expected: vec![Value::from(1), Value::from(2)],
    };

    let result = visit_array_ref(&array, visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![Value::from(1), Value::from(2)]);
}

#[test]
#[should_panic(expected = "fewer elements in array")]
fn test_visit_array_ref_panic() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error, SeqRefDeserializer};

    struct TestVisitor {
        expected: Vec<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<S>(self, _seq: &mut S) -> Result<Self::Value, Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Err(de::Error::custom("Not enough elements"))
        }
    }

    let array = vec![Value::from(1)];
    let visitor = TestVisitor {
        expected: vec![Value::from(1)],
    };

    let _result = visit_array_ref(&array, visitor);
}

#[test]
fn test_visit_array_ref_remaining_not_zero() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error, SeqRefDeserializer};

    struct TestVisitor {
        expected: Vec<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<S>(self, _seq: &mut S) -> Result<Self::Value, Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Ok(self.expected.clone())
        }
    }

    let array = vec![Value::from(1)];
    let visitor = TestVisitor {
        expected: vec![Value::from(1)],
    };

    let result = visit_array_ref(&array, visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![Value::from(1)]);
}

