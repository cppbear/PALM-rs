// Answer 0

#[test]
fn test_deserialize_any_null() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();
        
        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    let input = r#"null"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let result = deserializer.deserialize_any(Visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_true() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;
        
        fn visit_bool(self, v: bool) -> Result<Self::Value> {
            Ok(v)
        }
    }

    let input = r#"true"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let result = deserializer.deserialize_any(Visitor);
    assert_eq!(result.ok(), Some(true));
}

#[test]
fn test_deserialize_any_false() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;
        
        fn visit_bool(self, v: bool) -> Result<Self::Value> {
            Ok(v)
        }
    }

    let input = r#"false"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let result = deserializer.deserialize_any(Visitor);
    assert_eq!(result.ok(), Some(false));
}

#[test]
fn test_deserialize_any_number() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = i64;
        
        fn visit_i64(self, v: i64) -> Result<Self::Value> {
            Ok(v)
        }
    }

    let input = r#"42"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let result = deserializer.deserialize_any(Visitor);
    assert_eq!(result.ok(), Some(42));
}

#[test]
fn test_deserialize_any_string() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = String;
        
        fn visit_str(self, v: &'de str) -> Result<Self::Value> {
            Ok(v.to_owned())
        }
    }

    let input = r#""hello""#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let result = deserializer.deserialize_any(Visitor);
    assert_eq!(result.ok(), Some("hello".to_owned()));
}

#[test]
fn test_deserialize_any_array() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Vec<u32>;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            let mut vec = vec![];
            while let Some(elem) = seq.next_element::<u32>()? {
                vec.push(elem);
            }
            Ok(vec)
        }
    }

    let input = r#"[1, 2, 3]"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let result = deserializer.deserialize_any(Visitor);
    assert_eq!(result.ok(), Some(vec![1, 2, 3]));
}

#[test]
fn test_deserialize_any_object() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = std::collections::HashMap<String, String>;

        fn visit_map<V>(self, mut map: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            let mut hm = std::collections::HashMap::new();
            while let Some((key, value)) = map.next_entry::<String, String>()? {
                hm.insert(key, value);
            }
            Ok(hm)
        }
    }

    let input = r#"{"key": "value"}"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let result = deserializer.deserialize_any(Visitor);
    let mut expected = std::collections::HashMap::new();
    expected.insert("key".to_owned(), "value".to_owned());
    assert_eq!(result.ok(), Some(expected));
}

