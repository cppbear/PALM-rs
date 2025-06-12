// Answer 0

#[test]
fn test_new_function() {
    struct SerializeStructVariantAsMapValue<M> {
        map: M,
        name: &'static str,
        fields: Vec<i32>,
    }
    
    // Here we use a simple HashMap as the generic type for `M`.
    use std::collections::HashMap;
    
    let map: HashMap<String, i32> = HashMap::new();
    let name: &'static str = "test_variant";
    let len: usize = 3;
    
    let instance = SerializeStructVariantAsMapValue::new(map, name, len);
    
    assert_eq!(instance.name, name);
    assert_eq!(instance.fields.capacity(), len);
}

#[test]
fn test_new_function_with_empty_map() {
    struct SerializeStructVariantAsMapValue<M> {
        map: M,
        name: &'static str,
        fields: Vec<i32>,
    }
    
    use std::collections::HashMap;
    
    let map: HashMap<String, i32> = HashMap::new();
    let name: &'static str = "empty_map_variant";
    let len: usize = 0;
    
    let instance = SerializeStructVariantAsMapValue::new(map, name, len);
    
    assert_eq!(instance.name, name);
    assert_eq!(instance.fields.capacity(), len);
}

