// Answer 0

#[test]
fn test_index_mut_insert_new_key_in_object() {
    use serde_json::json;
    
    let mut data = json!({});
    data["x"] = json!(1);
    
    assert_eq!(data["x"], json!(1));
}

#[test]
fn test_index_mut_replace_existing_key_in_object() {
    use serde_json::json;
    
    let mut data = json!({ "x": 0 });
    data["x"] = json!(1);
    
    assert_eq!(data["x"], json!(1));
}

#[test]
fn test_index_mut_insert_new_key_in_empty_object() {
    use serde_json::json;
    
    let mut data = json!(null);
    data["y"] = json!(2);
    
    assert_eq!(data["y"], json!(2));
}

#[test]
fn test_index_mut_insert_key_in_non_object() {
    use serde_json::json;
    
    let mut data = json!(42);
    
    let panic_result = std::panic::catch_unwind(|| {
        data["y"] = json!(true);
    });
    
    assert!(panic_result.is_err());
}

#[test]
fn test_index_mut_insert_into_array_valid_index() {
    use serde_json::json;

    let mut data = json!([1, 2, 3]);
    data[1] = json!(4);
    
    assert_eq!(data[1], json!(4));
}

#[test]
fn test_index_mut_insert_into_array_out_of_bounds() {
    use serde_json::json;

    let mut data = json!([1, 2, 3]);
    
    let panic_result = std::panic::catch_unwind(|| {
        data[5] = json!(4);
    });
    
    assert!(panic_result.is_err());
}

#[test]
fn test_index_mut_insert_into_non_array() {
    use serde_json::json;

    let mut data = json!(42);
    
    let panic_result = std::panic::catch_unwind(|| {
        data[0] = json!(5);
    });
    
    assert!(panic_result.is_err());
}

#[test]
fn test_index_mut_insert_into_nested_structure() {
    use serde_json::json;

    let mut data = json!({});
    data["a"]["b"]["c"] = json!(true);
    
    assert_eq!(data["a"]["b"]["c"], json!(true));
}

