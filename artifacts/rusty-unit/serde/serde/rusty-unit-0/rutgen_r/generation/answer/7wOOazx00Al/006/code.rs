// Answer 0

#[test]
fn test_visit_map_with_missing_end_field() {
    struct TestMapAccess {
        keys: Vec<Field>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            Err(serde::de::value::Error::custom("mock error"))
        }
    }

    let keys = vec![Field::End]; // This indicates an end field, but we won't provide a corresponding value.
    let access = TestMapAccess { keys, index: 0 };
    let result: Result<_, _> = visit_map(access);

    match result {
        Err(err) => assert_eq!(err.to_string(), "missing field `end`"),
        _ => panic!("Expected an error due to missing 'end' field"),
    }
}

#[test]
fn test_visit_map_with_duplicate_end_field() {
    struct TestMapAccess {
        keys: Vec<Field>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            Ok(0 as V) // Return a valid value for deserialization
        }
    }

    let keys = vec![Field::End, Field::End]; // Duplicate 'end' field
    let access = TestMapAccess { keys, index: 0 };
    let result: Result<_, _> = visit_map(access);

    match result {
        Err(err) => assert_eq!(err.to_string(), "duplicate field `end`"),
        _ => panic!("Expected an error due to duplicate 'end' field"),
    }
}

