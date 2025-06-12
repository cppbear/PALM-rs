// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize)]
    struct SimpleStruct {
        name: String,
        age: u32,
    }

    #[derive(Serialize)]
    struct ComplexStruct {
        items: std::collections::HashMap<String, String>,
    }

    #[test]
    fn test_to_string_pretty_simple() {
        let data = SimpleStruct {
            name: "Alice".to_string(),
            age: 30,
        };

        let result = to_string_pretty(&data).unwrap();
        assert!(result.contains("Alice"));
        assert!(result.contains("30"));
    }

    #[test]
    fn test_to_string_pretty_complex() {
        let mut items = std::collections::HashMap::new();
        items.insert("key1".to_string(), "value1".to_string());
        items.insert("key2".to_string(), "value2".to_string());

        let data = ComplexStruct { items };

        let result = to_string_pretty(&data).unwrap();
        assert!(result.contains("key1"));
        assert!(result.contains("value1"));
        assert!(result.contains("key2"));
        assert!(result.contains("value2"));
    }

    #[should_panic]
    fn test_to_string_pretty_non_string_key() {
        let mut items: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
        items.insert(1, "value1".to_string());

        #[derive(Serialize)]
        struct InvalidStruct {
            items: std::collections::HashMap<i32, String>,
        }

        let data = InvalidStruct { items };
        let _ = to_string_pretty(&data).unwrap();
    }
}

