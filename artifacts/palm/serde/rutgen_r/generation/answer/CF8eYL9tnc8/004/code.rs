// Answer 0

#[test]
fn test_deserialize_enum_invalid_map_too_many_keys() {
    struct TestVisitor;
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_enum<E>(self, _deserializer: E) -> Result<(), E::Error>
        where
            E: serde::de::EnumAccess<'de>,
        {
            Ok(())
        }
    }
    
    struct TestStruct {
        content: Content,
    }
    
    enum Content {
        Map(std::collections::BTreeMap<String, String>),
        String(String),
        Str(&'static str),
    }
    
    let mut map = std::collections::BTreeMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());

    let test_struct = TestStruct {
        content: Content::Map(map),
    };
    
    let result = test_struct.deserialize_enum("TestEnum", &["key1", "key2"], TestVisitor);
    
    match result {
        Err(ref err) if err == &serde::de::Error::invalid_value(
            serde::de::Unexpected::Map,
            &"map with a single key",
        ) => {}
        _ => panic!("Expected Err with invalid_value"),
    }
}

