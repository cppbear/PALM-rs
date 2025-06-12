// Answer 0

#[test]
fn test_next_value_seed_with_valid_content() {
    use crate::de::{DeserializeSeed, Visitor, Error};
    use std::marker::PhantomData;

    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = &'de str;

        fn deserialize<V>(self, visitor: V) -> Result<Self::Value, Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_str("test").map_err(|_| Error::custom("visit failed"))
        }
    }

    let content_value = crate::Content::Str("valid");
    
    let mut flat_map_access = crate::FlatMapAccess {
        iter: vec![Some((content_value.clone(), content_value))].into_iter(),
        pending_content: None,
        _marker: PhantomData,
    };

    flat_map_access.pending_content = Some(&content_value);
    
    let result: Result<&str, _> = flat_map_access.next_value_seed(DummySeed);
    
    assert_eq!(result, Ok("valid"));
}

#[test]
#[should_panic(expected = "value is missing")]
fn test_next_value_seed_with_none_content() {
    use crate::de::{DeserializeSeed, Visitor, Error};
    use std::marker::PhantomData;

    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = &'de str;

        fn deserialize<V>(self, visitor: V) -> Result<Self::Value, Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_str("test").map_err(|_| Error::custom("visit failed"))
        }
    }

    let mut flat_map_access = crate::FlatMapAccess {
        iter: vec![None].into_iter(),
        pending_content: None,
        _marker: PhantomData,
    };

    // No content is set in pending_content, this should trigger a panic.
    let _result: Result<&str, _> = flat_map_access.next_value_seed(DummySeed);
}

