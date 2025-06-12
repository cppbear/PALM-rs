// Answer 0

#[test]
fn test_visit_array_fewer_elements() {
    struct TestVisitor {
        count: usize,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn visit_seq<S>(self, _: &mut S) -> Result<Self::Value, serde::de::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Ok(vec![1, 2, 3]) // returning a value that expects more elements
        }
    }

    let input_array = vec![1, 2]; // fewer elements than expected by visitor
    let visitor = TestVisitor { count: 3 };

    let result = visit_array(input_array, visitor);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, serde::de::Error::invalid_length(2, &"fewer elements in array"));
    }
}

#[test]
fn test_visit_array_exact_elements() {
    struct TestVisitor {
        count: usize,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn visit_seq<S>(self, _: &mut S) -> Result<Self::Value, serde::de::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Ok(vec![1, 2]) // returning the exact number of elements
        }
    }

    let input_array = vec![1, 2]; // exact number of elements
    let visitor = TestVisitor { count: 2 };

    let result = visit_array(input_array, visitor);
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value, vec![1, 2]);
    }
}

