// Answer 0

#[test]
fn test_map_fmt_empty() {
    use std::fmt::Formatter;
    
    let map = Map { map: MapImpl::new() };
    let mut formatter = Formatter::new();
    
    // Capturing the result of the formatting.
    let result = map.fmt(&mut formatter);
    
    assert!(result.is_ok());
}

#[test]
fn test_map_fmt_with_entries() {
    use std::fmt::Formatter;
    
    let mut map_impl = MapImpl::new();
    map_impl.insert("key1".to_string(), Value::Bool(true));
    map_impl.insert("key2".to_string(), Value::Number(serde_json::Number::from(42)));
    
    let map = Map { map: map_impl };
    let mut formatter = Formatter::new();
    
    // Capturing the result of the formatting.
    let result = map.fmt(&mut formatter);
    
    assert!(result.is_ok());
}

#[test]
fn test_map_fmt_with_null_value() {
    use std::fmt::Formatter;
    
    let mut map_impl = MapImpl::new();
    map_impl.insert("key1".to_string(), Value::Null);
    
    let map = Map { map: map_impl };
    let mut formatter = Formatter::new();
    
    // Capturing the result of the formatting.
    let result = map.fmt(&mut formatter);
    
    assert!(result.is_ok());
}

