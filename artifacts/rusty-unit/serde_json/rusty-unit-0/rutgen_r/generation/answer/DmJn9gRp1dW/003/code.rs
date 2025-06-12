// Answer 0

#[derive(Default)]
struct TestVisitor {
    value: i32,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = Vec<i32>;

    fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        Ok(vec![self.value])
    }
}

#[test]
fn test_visit_array_valid() {
    let array = vec![Value::Number(1.into()), Value::Number(2.into())];
    let visitor = TestVisitor { value: 1 };
    let result = visit_array(array, visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1]);
}

#[test]
fn test_visit_array_invalid_length() {
    let array = vec![Value::Number(1.into())];
    let visitor = TestVisitor { value: 1 };
    let result = visit_array(array, visitor);
    assert!(result.is_err());
    
    if let Err(serde::de::Error::InvalidLength { .. }) = result {
        // Test is successful if error is of type InvalidLength
    } else {
        panic!("Expected InvaildLength error, but got: {:?}", result);
    }
}

#[test]
#[should_panic]
fn test_visit_array_empty() {
    let array: Vec<Value> = vec![];
    let visitor = TestVisitor { value: 1 };
    visit_array(array, visitor).unwrap();
}

