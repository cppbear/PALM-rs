// Answer 0

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_json::to_string;

    #[derive(Serialize)]
    struct InvalidMapKey {
        data: std::collections::HashMap<String, i32>,
    }

    #[test]
    fn test_to_string_err_invalid_key() {
        let mut map = std::collections::HashMap::new();
        // Using a non-string key (usize is used as a key, which is invalid for serialization)
        map.insert(1usize, 42);
        
        let value = InvalidMapKey { data: map };
        let result = to_string(&value);
        
        assert!(result.is_err());
    }

    #[derive(Serialize)]
    struct UnserializableData;

    #[test]
    fn test_to_string_err_unserializable() {
        // Creating a type that doesn't implement `Serialize`
        let value = UnserializableData;
        let result = to_string(&value);
        
        assert!(result.is_err());
    }
}

