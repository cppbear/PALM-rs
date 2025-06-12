// Answer 0

#[test]
fn test_size_hint_with_non_matching_constraints() {
    struct TestValue {
        iter: vec::IntoIter<Value>,
    }

    impl SeqAccess<'static> for TestValue {
        type Error = Error;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Error>
        where
            T: DeserializeSeed<'static>,
        {
            Ok(None)
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let values = vec![Value::Null, Value::Bool(true), Value::Number(Number::from(42))];
    let test_value = TestValue {
        iter: values.into_iter(),
    };

    assert_eq!(test_value.size_hint(), None);
}

