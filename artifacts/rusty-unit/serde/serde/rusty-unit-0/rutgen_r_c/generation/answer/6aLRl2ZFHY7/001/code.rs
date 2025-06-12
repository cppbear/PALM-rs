// Answer 0

fn test_next_value_seed_with_some_content() {
    use crate::de::{DeserializeSeed, Visitor, Error};

    struct TestDeserializer;
    
    impl<'de> DeserializeSeed<'de> for TestDeserializer {
        type Value = String;

        fn deserialize<Visitor: crate::de::Visitor<'de>>(self, visitor: Visitor) -> Result<Self::Value, Error> {
            visitor.visit_string("test".to_string())
        }
    }

    struct TestMapAccess<'a> {
        pending_content: Option<Content<'a>>,
    }

    impl<'a> TestMapAccess<'a> {
        fn new(content: Content<'a>) -> Self {
            TestMapAccess {
                pending_content: Some(content),
            }
        }

        fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Error>
        where
            T: DeserializeSeed<'a>,
        {
            match self.pending_content.take() {
                Some(value) => seed.deserialize(ContentDeserializer::new(value)),
                None => Err(Error::custom("value is missing")),
            }
        }
    }

    let content_value = Content::String("Example".to_string());
    let mut map_access = TestMapAccess::new(content_value);

    let result: Result<String, Error> = map_access.next_value_seed(TestDeserializer);

    assert_eq!(result.unwrap(), "test");
}

fn test_next_value_seed_with_none_content() {
    use crate::de::{DeserializeSeed, Visitor, Error};

    struct TestDeserializer;

    impl<'de> DeserializeSeed<'de> for TestDeserializer {
        type Value = String;

        fn deserialize<Visitor: crate::de::Visitor<'de>>(self, _visitor: Visitor) -> Result<Self::Value, Error> {
            Err(Error::custom("should not be called"))
        }
    }

    struct TestMapAccess<'a> {
        pending_content: Option<Content<'a>>,
    }

    impl<'a> TestMapAccess<'a> {
        fn new() -> Self {
            TestMapAccess {
                pending_content: None,
            }
        }

        fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Error>
        where
            T: DeserializeSeed<'a>,
        {
            match self.pending_content.take() {
                Some(value) => seed.deserialize(ContentDeserializer::new(value)),
                None => Err(Error::custom("value is missing")),
            }
        }
    }

    let mut map_access = TestMapAccess::new();
    let result: Result<String, Error> = map_access.next_value_seed(TestDeserializer);

    assert!(result.is_err());
}

