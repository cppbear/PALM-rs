// Answer 0

#[test]
fn test_into_deserializer() {
    struct DummyMapAccess;
    
    impl<'de> de::MapAccess<'de> for DummyMapAccess {
        type Error = ();
        
        fn next_entry<V>(&mut self) -> Result<Option<(V::Value, V::Value)>, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            Ok(None)
        }
    }

    let deserializer = MapAccessDeserializer {
        map: DummyMapAccess,
    };
    
    let result = deserializer.into_deserializer();
    assert_eq!(std::ptr::eq(&deserializer, &result), true);
}

#[test]
fn test_into_deserializer_with_different_instance() {
    struct AnotherDummyMapAccess;

    impl<'de> de::MapAccess<'de> for AnotherDummyMapAccess {
        type Error = ();
        
        fn next_entry<V>(&mut self) -> Result<Option<(V::Value, V::Value)>, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            Ok(None)
        }
    }

    let deserializer_1 = MapAccessDeserializer {
        map: DummyMapAccess,
    };
    
    let deserializer_2 = MapAccessDeserializer {
        map: AnotherDummyMapAccess,
    };

    let result = deserializer_1.into_deserializer();
    assert_eq!(std::ptr::eq(&deserializer_1, &result), true);

    let result_2 = deserializer_2.into_deserializer();
    assert_eq!(std::ptr::eq(&deserializer_2, &result_2), true);
}

