// Answer 0

#[test]
fn test_next_value_seed_with_some_value() {
    use crate::de::{DeserializeSeed, Visitor, Error};
    
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, Self::Error>
        where
            T: Deserializer<'de>,
        {
            Ok(String::from("test_value"))
        }
    }

    let content = Content::String(String::from("test_value"));
    let mut map_access = FlatMapAccess {
        iter: vec![Some((Content::Str("key"), content))].iter(),
        pending_content: None,
        _marker: std::marker::PhantomData,
    };

    let result = map_access.next_value_seed(TestSeed);

    assert_eq!(result.unwrap(), "test_value");
}

#[test]
fn test_next_value_seed_with_none_value() {
    use crate::de::{DeserializeSeed, Visitor, Error};
    
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, Self::Error>
        where
            T: Deserializer<'de>,
        {
            Ok(String::from("test_value"))
        }
    }

    let content = Content::String(String::from("test_value"));
    let mut map_access = FlatMapAccess {
        iter: vec![Some((Content::Str("key"), content))].iter(),
        pending_content: Some(&content),
        _marker: std::marker::PhantomData,
    };

    let _ = map_access.next_key_seed(TestSeed);  // consume pending content

    let result = map_access.next_value_seed(TestSeed);

    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "value is missing")]
fn test_next_value_seed_should_panic() {
    use crate::de::{DeserializeSeed, Visitor, Error};

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, Self::Error>
        where
            T: Deserializer<'de>,
        {
            Ok(String::from("test_value"))
        }
    }

    let mut map_access = FlatMapAccess {
        iter: vec![].iter(),
        pending_content: None,
        _marker: std::marker::PhantomData,
    };

    let _ = map_access.next_value_seed(TestSeed).unwrap();
}

