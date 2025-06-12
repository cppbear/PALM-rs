// Answer 0

#[test]
fn test_visit_array_with_error_from_visitor() {
    struct ErrorVisitor;
    
    impl<'de> Visitor<'de> for ErrorVisitor {
        type Value = ();

        fn visit_seq<T>(&mut self, _: &mut T) -> Result<Self::Value, Error>
        where
            T: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("error during visiting sequence"))
        }
    }

    let array = vec![Value::String("value1".to_string()), Value::String("value2".to_string())];
    let visitor = ErrorVisitor;

    let result = visit_array(array, visitor);
    assert!(result.is_err());
}

#[test]
fn test_visit_array_with_invalid_length() {
    struct ValidVisitor {
        count: usize,
    }

    impl<'de> Visitor<'de> for ValidVisitor {
        type Value = Vec<()>;

        fn visit_seq<T>(&mut self, _: &mut T) -> Result<Self::Value, Error>
        where
            T: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("visitor error"))
        }
    }

    let array = vec![Value::String("value1".to_string()), Value::String("value2".to_string())];
    let visitor = ValidVisitor { count: 2 };

    let result = visit_array(array, visitor);
    assert!(result.is_err());
}

