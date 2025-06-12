// Answer 0

#[test]
fn test_clear_with_some_map() {
    let mut ext = Extensions::new();
    ext.map = Some(Box::new(HashMap::new()));
    ext.insert(5i32);
    ext.clear();
}

#[test]
fn test_clear_with_some_map_and_multiple_insertions() {
    let mut ext = Extensions::new();
    ext.map = Some(Box::new(HashMap::new()));
    ext.insert(10i32);
    ext.insert("Hello".to_string());
    ext.clear();
}

#[test]
fn test_clear_after_single_insertion() {
    let mut ext = Extensions::new();
    ext.map = Some(Box::new(HashMap::new()));
    ext.insert(42u32);
    ext.clear();
}

#[test]
fn test_clear_when_map_is_not_empty() {
    let mut ext = Extensions::new();
    ext.map = Some(Box::new(HashMap::new()));
    ext.insert(99i64);
    ext.clear();
}

#[test]
fn test_clear_on_empty_map() {
    let mut ext = Extensions::new();
    ext.map = Some(Box::new(HashMap::new()));
    ext.clear();
}

#[test]
fn test_clear_with_scoped_map_insert() {
    let mut ext = Extensions::new();
    ext.map = Some(Box::new(HashMap::new()));
    ext.insert(3.14f64);
    ext.clear();
}

