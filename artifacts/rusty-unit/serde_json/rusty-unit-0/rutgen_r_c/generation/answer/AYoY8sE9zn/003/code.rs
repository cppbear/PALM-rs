// Answer 0

#[test]
fn test_size_hint_lower_not_equal_upper() {
    struct TestMapDeserializer {
        iter: std::iter::Empty<(usize, Option<usize>)>,
    }

    impl MapAccess<'static> for TestMapDeserializer {
        type Error = Error;

        fn next_key_seed<T>(&mut self, _: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'static>,
        {
            Ok(None)
        }

        fn next_value_seed<T>(&mut self, _: T) -> Result<T::Value, Self::Error>
        where
            T: DeserializeSeed<'static>,
        {
            unimplemented!()
        }

        fn size_hint(&self) -> Option<usize> {
            match (0, Some(1)) {
                (lower, Some(upper)) if lower != upper => Some(upper),
                _ => None,
            }
        }
    }

    let deserializer = TestMapDeserializer {
        iter: std::iter::empty(),
    };

    let result = deserializer.size_hint();
    assert_eq!(result, None);
}

#[test]
fn test_size_hint_empty_iter() {
    struct TestMapDeserializer {
        iter: std::iter::Empty<(usize, Option<usize>)>,
    }

    impl MapAccess<'static> for TestMapDeserializer {
        type Error = Error;

        fn next_key_seed<T>(&mut self, _: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'static>,
        {
            Ok(None)
        }

        fn next_value_seed<T>(&mut self, _: T) -> Result<T::Value, Self::Error>
        where
            T: DeserializeSeed<'static>,
        {
            unimplemented!()
        }

        fn size_hint(&self) -> Option<usize> {
            match (0, None) {
                (lower, Some(upper)) if lower != upper => Some(upper),
                _ => None,
            }
        }
    }

    let deserializer = TestMapDeserializer {
        iter: std::iter::empty(),
    };

    let result = deserializer.size_hint();
    assert_eq!(result, None);
}

