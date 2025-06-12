// Answer 0

#[test]
fn test_next_value_seed_with_none_pending_content() {
    use crate::de::{Error, DeserializeSeed};
    use std::marker::PhantomData;

    struct TestError;
    impl Error for TestError {}

    struct TestSeed;
    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = u32;

        fn deserialize<D>(self, _: D) -> Result<Self::Value, Self::Error>
        where
            D: Deserializer<'de>,
        {
            // This method should not be called as we expect an error.
            unreachable!()
        }
    }

    let mut flat_map_access: FlatMapAccess<'_, '_, TestError> = FlatMapAccess {
        iter: &mut [].iter(),
        pending_content: None,
        _marker: PhantomData,
    };

    let result: Result<u32, TestError> = flat_map_access.next_value_seed(TestSeed);
    assert!(result.is_err());
    match result {
        Err(e) => assert_eq!(e.to_string(), "value is missing"),
        _ => panic!("Expected an error"),
    }
}

