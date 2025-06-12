// Answer 0

#[derive(Debug)]
struct SerializeStructVariantAsMapValue<M> {
    map: M,
    name: &'static str,
    fields: Vec<String>,
}

impl<M> SerializeStructVariantAsMapValue<M> {
    pub fn new(map: M, name: &'static str, len: usize) -> Self {
        SerializeStructVariantAsMapValue {
            map,
            name,
            fields: Vec::with_capacity(len),
        }
    }
}

#[test]
fn test_new_with_empty_map_and_zero_length() {
    let map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let name = "test_variant";
    let len = 0;

    let result = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.len(), len);
}

#[test]
fn test_new_with_empty_map_and_non_zero_length() {
    let map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let name = "test_variant";
    let len = 5;

    let result = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.len(), 0);  // should be initialized to capacity of 5
}

#[test]
fn test_new_with_non_empty_map_and_zero_length() {
    let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    let name = "test_variant";
    let len = 0;

    let result = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.len(), len);
}

#[test]
fn test_new_with_non_empty_map_and_non_zero_length() {
    let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    let name = "test_variant";
    let len = 3;

    let result = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.len(), 0);  // should be initialized to capacity of 3
}

#[test]
fn test_new_with_large_length() {
    let map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let name = "test_variant";
    let len = 1000;

    let result = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.len(), 0);  // should be initialized to capacity of 1000
}

