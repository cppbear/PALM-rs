// Answer 0

#[test]
fn test_visit_array_empty() {
    struct MockVisitor {
        count: usize,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = usize;

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, Error>
        where
            A: SeqAccess<'de>,
        {
            Ok(self.count)
        }
    }

    let array: Vec<Value> = Vec::new();
    let result = visit_array(array, MockVisitor { count: 0 });
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_visit_array_one_element() {
    struct MockVisitor {
        count: usize,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = usize;

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, Error>
        where
            A: SeqAccess<'de>,
        {
            Ok(self.count + 1)
        }
    }

    let array = vec![Value::Bool(true)];
    let result = visit_array(array, MockVisitor { count: 0 });
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_visit_array_multiple_elements() {
    struct MockVisitor {
        count: usize,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = usize;

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, Error>
        where
            A: SeqAccess<'de>,
        {
            Ok(self.count + 3)
        }
    }

    let array = vec![Value::Bool(true), Value::Bool(false), Value::Null];
    let result = visit_array(array, MockVisitor { count: 0 });
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn test_visit_array_invalid_length() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, Error>
        where
            A: SeqAccess<'de>,
        {
            Err(Error::syntax(ErrorCode::ExpectedNumericKey, 0, 0))
        }
    }

    let array = vec![Value::Bool(true)];
    let result = visit_array(array, MockVisitor);
    assert!(result.is_err());
}

