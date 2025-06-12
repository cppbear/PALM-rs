// Answer 0

#[test]
fn test_new_with_valid_map_and_name() {
    struct DummyMap {
        data: std::collections::HashMap<String, i32>,
    }
    
    let map = DummyMap {
        data: std::collections::HashMap::new(),
    };
    let name = "test_variant";
    let len = 5;

    let result = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.capacity(), len);
}

#[test]
fn test_new_with_empty_name() {
    struct DummyMap {
        data: std::collections::HashMap<String, i32>,
    }
    
    let map = DummyMap {
        data: std::collections::HashMap::new(),
    };
    let name = "";
    let len = 10;

    let result = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.capacity(), len);
}

#[test]
fn test_new_with_zero_length() {
    struct DummyMap {
        data: std::collections::HashMap<String, i32>,
    }

    let map = DummyMap {
        data: std::collections::HashMap::new(),
    };
    let name = "zero_length";
    let len = 0;

    let result = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.capacity(), len);
}

