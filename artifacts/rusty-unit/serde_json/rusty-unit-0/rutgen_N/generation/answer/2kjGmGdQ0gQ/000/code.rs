// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn test_to_string_with_basic_struct() {
        #[derive(Serialize)]
        struct TestStruct {
            name: String,
            age: u32,
        }
        
        let test_value = TestStruct {
            name: "Alice".to_string(),
            age: 30,
        };

        let result = to_string(&test_value).unwrap();
        assert_eq!(result, r#"{"name":"Alice","age":30}"#);
    }

    #[test]
    fn test_to_string_with_complex_struct() {
        #[derive(Serialize)]
        struct ComplexStruct {
            id: u32,
            tags: Vec<String>,
        }

        let test_value = ComplexStruct {
            id: 1,
            tags: vec!["rust".to_string(), "serde".to_string()],
        };

        let result = to_string(&test_value).unwrap();
        assert_eq!(result, r#"{"id":1,"tags":["rust","serde"]}"#);
    }

    #[test]
    #[should_panic]
    fn test_to_string_with_invalid_serialization() {
        #[derive(Serialize)]
        struct InvalidStruct {
            map: std::collections::HashMap<u32, String>,
        }

        let mut map = std::collections::HashMap::new();
        map.insert(1, "One".to_string());
        let test_value = InvalidStruct { map };

        let _result = to_string(&test_value).unwrap();
    }
}

