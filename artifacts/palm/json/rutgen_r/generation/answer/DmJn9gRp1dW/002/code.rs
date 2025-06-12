// Answer 0

#[test]
fn test_visit_array_success() {
    struct TestVisitor {
        value: Option<i32>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = i32;

        fn visit_seq<V>(self, seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            if let Some(value) = seq.next_element::<i32>()? {
                Ok(value)
            } else {
                Err(serde::de::Error::invalid_length(0, &"expected a value"))
            }
        }
    }

    let array = vec![serde_json::Value::Number(5.into())];
    let visitor = TestVisitor { value: None };
    
    let result = visit_array(array, visitor);
    assert_eq!(result, Ok(5));
}

#[test]
#[should_panic(expected = "fewer elements in array")]
fn test_visit_array_failure_remaining_not_zero() {
    struct PanicVisitor;

    impl<'de> serde::de::Visitor<'de> for PanicVisitor {
        type Value = ();

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            // This will make remaining not zero
            Err(serde::de::Error::invalid_length(1, &"fewer elements in array"))
        }
    }

    let array = vec![serde_json::Value::Number(5.into()), serde_json::Value::Number(6.into())];
    let visitor = PanicVisitor;

    let _ = visit_array(array, visitor);
}

