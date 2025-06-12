// Answer 0

#[test]
fn test_size_hint_equal_bounds() {
    struct TestValueIterator {
        values: Vec<Value>,
    }

    impl TestValueIterator {
        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.values.len();
            (len, Some(len))
        }
    }

    struct TestSeqRefDeserializer<'de> {
        iter: TestValueIterator,
    }

    impl<'de> SeqAccess<'de> for TestSeqRefDeserializer<'de> {
        type Error = Error;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Error>
        where
            T: DeserializeSeed<'de>,
        {
            unimplemented!()
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_values = vec![Value::Null, Value::Bool(true), Value::Number(Number::from(1))];
    let iter = TestValueIterator { values: test_values };
    let deserializer = TestSeqRefDeserializer { iter };

    assert_eq!(deserializer.size_hint(), Some(3));
}

