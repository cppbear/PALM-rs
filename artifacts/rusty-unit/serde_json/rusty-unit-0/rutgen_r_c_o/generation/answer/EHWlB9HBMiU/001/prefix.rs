// Answer 0

#[test]
fn test_retain_empty() {
    let mut map = Map::new();
    map.retain(|_k, _v| true);
}

#[test]
fn test_retain_single_element_keep() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.retain(|_k, _v| true);
}

#[test]
fn test_retain_single_element_remove() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.retain(|_k, _v| false);
}

#[test]
fn test_retain_multiple_elements_keep() {
    let mut map = Map::with_capacity(10);
    map.insert("key1".to_string(), Value::Number(42.into()));
    map.insert("key2".to_string(), Value::String("value".to_string()));
    map.retain(|_k, _v| true);
}

#[test]
fn test_retain_multiple_elements_partial_remove() {
    let mut map = Map::with_capacity(10);
    map.insert("key1".to_string(), Value::Number(42.into()));
    map.insert("key2".to_string(), Value::String("value".to_string()));
    map.retain(|k, _v| k != "key1");
}

#[test]
fn test_retain_large_map() {
    let mut map = Map::with_capacity(1_000);
    for i in 0..1_000 {
        map.insert(format!("key{}", i), Value::Number(i.into()));
    }
    map.retain(|k, _v| k.contains('5'));
}

#[test]
fn test_retain_mixed_values() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.insert("key2".to_string(), Value::Bool(false));
    map.insert("key3".to_string(), Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]));
    map.retain(|k, v| if k == "key3" { v.is_array() } else { false });
}

#[test]
fn test_retain_single_character_keys() {
    let mut map = Map::new();
    map.insert("a".to_string(), Value::String("a value".to_string()));
    map.insert("b".to_string(), Value::Num(5.into()));
    map.retain(|k, _v| k.len() == 1);
}

