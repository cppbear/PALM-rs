// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    
    #[derive(Serialize)]
    struct SimpleStruct {
        field1: String,
        field2: i32,
    }

    #[derive(Serialize)]
    struct NestedStruct {
        field1: String,
        field2: SimpleStruct,
    }

    #[test]
    fn test_serialize_simple_struct() {
        let data = SimpleStruct {
            field1: "test".to_string(),
            field2: 42,
        };
        
        let result = to_vec(&data);
        assert!(result.is_ok());
        
        let json_bytes = result.unwrap();
        let json_str = String::from_utf8(json_bytes).unwrap();
        assert_eq!(json_str, r#"{"field1":"test","field2":42}"#);
    }

    #[test]
    fn test_serialize_nested_struct() {
        let data = NestedStruct {
            field1: "outer".to_string(),
            field2: SimpleStruct {
                field1: "inner".to_string(),
                field2: 100,
            },
        };

        let result = to_vec(&data);
        assert!(result.is_ok());

        let json_bytes = result.unwrap();
        let json_str = String::from_utf8(json_bytes).unwrap();
        assert_eq!(json_str, r#"{"field1":"outer","field2":{"field1":"inner","field2":100}}"#);
    }

    #[derive(Serialize)]
    struct StructWithNonStringKey {
        #[serde(serialize_with = "non_string_key_map")]
        map: std::collections::HashMap<i32, String>,
    }

    fn non_string_key_map<S>(value: &std::collections::HashMap<i32, String>, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(serde::ser::Error::custom("Non-string keys not allowed"))
    }

    #[test]
    #[should_panic(expected = "Non-string keys not allowed")]
    fn test_serialize_struct_with_non_string_keys() {
        let mut map = std::collections::HashMap::new();
        map.insert(1, "value".to_string());

        let data = StructWithNonStringKey { map };
        
        let _result = to_vec(&data).unwrap();
    }
}

