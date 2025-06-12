// Answer 0

#[test]
fn test_index_into_mut_valid_index() {
    struct Index(usize);
    
    let index = Index(1);
    let mut value = serde_json::Value::Array(vec![
        serde_json::Value::Number(serde_json::Number::from(1)),
        serde_json::Value::Number(serde_json::Number::from(2)),
        serde_json::Value::Number(serde_json::Number::from(3)),
    ]);
    
    let result = index.index_into_mut(&mut value);
    assert!(result.is_some());
    assert_eq!(result, Some(&mut serde_json::Value::Number(serde_json::Number::from(2))));
}

#[test]
fn test_index_into_mut_out_of_bounds() {
    struct Index(usize);
    
    let index = Index(5);
    let mut value = serde_json::Value::Array(vec![
        serde_json::Value::Number(serde_json::Number::from(1)),
        serde_json::Value::Number(serde_json::Number::from(2)),
    ]);
    
    let result = index.index_into_mut(&mut value);
    assert!(result.is_none());
}

#[test]
fn test_index_into_mut_not_array() {
    struct Index(usize);
    
    let index = Index(0);
    let mut value = serde_json::Value::Object(serde_json::Map::new());
    
    let result = index.index_into_mut(&mut value);
    assert!(result.is_none());
}

