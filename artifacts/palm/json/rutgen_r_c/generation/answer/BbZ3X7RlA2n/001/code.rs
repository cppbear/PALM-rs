// Answer 0

#[test]
fn test_fmt_with_empty_map() {
    let map: Map<String, Value> = Map { map: MapImpl::<String, Value>::new() };
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| map.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "{}"); // Expect an empty map representation
}

#[test]
fn test_fmt_with_single_entry() {
    let mut map: Map<String, Value> = Map { map: MapImpl::<String, Value>::new() };
    map.map.insert("key".to_string(), Value::String("value".to_string()));
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| map.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "{\"key\":\"value\"}"); // Expect single entry map
}

#[test]
fn test_fmt_with_multiple_entries() {
    let mut map: Map<String, Value> = Map { map: MapImpl::<String, Value>::new() };
    map.map.insert("key1".to_string(), Value::Number(Number::from(1)));
    map.map.insert("key2".to_string(), Value::Bool(true));
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| map.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "{\"key1\":1,\"key2\":true}"); // Expect serialized multiple entries
}

#[test]
fn test_fmt_with_null_value() {
    let mut map: Map<String, Value> = Map { map: MapImpl::<String, Value>::new() };
    map.map.insert("null_key".to_string(), Value::Null);
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| map.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "{\"null_key\":null}"); // Expect representation of null value
}

#[test]
fn test_fmt_with_complex_object() {
    let mut inner_map: Map<String, Value> = Map { map: MapImpl::<String, Value>::new() };
    inner_map.map.insert("inner_key".to_string(), Value::String("inner_value".to_string()));
    
    let mut map: Map<String, Value> = Map { map: MapImpl::<String, Value>::new() };
    map.map.insert("outer_key".to_string(), Value::Object(inner_map));
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| map.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "{\"outer_key\":{\"inner_key\":\"inner_value\"}}"); // Expect serialization of complex object
}

