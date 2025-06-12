// Answer 0

#[test]
fn test_sort_keys_empty() {
    let mut map = Map::new();
    map.sort_keys();
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_single_entry() {
    let mut map = Map::new();
    map.insert("b".to_string(), Value::Number(1.into()));
    map.sort_keys();
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_already_sorted() {
    let mut map = Map::new();
    map.insert("a".to_string(), Value::Number(1.into()));
    map.insert("b".to_string(), Value::Number(2.into()));
    map.sort_keys();
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_reverse_sorted() {
    let mut map = Map::new();
    map.insert("c".to_string(), Value::Number(3.into()));
    map.insert("b".to_string(), Value::Number(2.into()));
    map.insert("a".to_string(), Value::Number(1.into()));
    map.sort_keys();
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_mixed_length_keys() {
    let mut map = Map::new();
    map.insert("abc".to_string(), Value::Number(3.into()));
    map.insert("a".to_string(), Value::Number(1.into()));
    map.insert("ab".to_string(), Value::Number(2.into()));
    map.sort_keys();
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_with_large_capacity() {
    let mut map = Map::with_capacity(1000);
    for i in (0..1000).rev() {
        map.insert(i.to_string(), Value::Number(i.into()));
    }
    map.sort_keys();
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_edge_cases() {
    let mut map = Map::new();
    map.insert("z".to_string(), Value::Number(1.into()));
    map.insert("y".to_string(), Value::Number(2.into()));
    map.insert("x".to_string(), Value::Number(3.into()));
    map.insert("a".to_string(), Value::Number(4.into()));
    map.insert("m".to_string(), Value::Number(5.into()));
    map.sort_keys();
}

