// Answer 0

#[test]
fn test_next_value_with_valid_deserialization() {
    struct ValidDeserializer {
        called: bool,
    }

    impl<'de> MapAccess<'de> for ValidDeserializer {
        type Error = TestError;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(Some(PhantomData))
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            self.called = true;
            Ok("value".to_string() as V::Value)
        }
    }

    struct TestError;
    impl Error for TestError {
        fn custom<T>(msg: T) -> Self where T: Display {
            TestError
        }
    }

    let mut valid_deserializer = ValidDeserializer { called: false };
    let result: Result<String, TestError> = valid_deserializer.next_value();
    assert!(valid_deserializer.called);
    assert_eq!(result.unwrap(), "value".to_string());
}

#[test]
#[should_panic]
fn test_next_value_called_before_next_key() {
    struct InvalidDeserializer {}

    impl<'de> MapAccess<'de> for InvalidDeserializer {
        type Error = TestError;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            panic!("next_key must be called before next_value");
        }
    }

    let mut invalid_deserializer = InvalidDeserializer {};
    let _: Result<(), TestError> = invalid_deserializer.next_value();
}

