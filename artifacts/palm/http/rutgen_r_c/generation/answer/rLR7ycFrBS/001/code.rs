// Answer 0

#[test]
fn test_get_empty() {
    let ext = Extensions::new();
    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_get_insert_and_retrieve() {
    let mut ext = Extensions::new();
    ext.insert(10i32);
    assert_eq!(ext.get::<i32>(), Some(&10i32));
}

#[test]
fn test_get_with_different_types() {
    let mut ext = Extensions::new();
    ext.insert(42i32);
    ext.insert("Hello".to_string());
    
    assert_eq!(ext.get::<i32>(), Some(&42i32));
    assert_eq!(ext.get::<String>(), Some(&"Hello".to_string()));
    assert!(ext.get::<f64>().is_none());
}

#[test]
fn test_get_after_remove() {
    let mut ext = Extensions::new();
    ext.insert(7i32);
    ext.remove::<i32>();
    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_get_with_empty_extensions_after_clear() {
    let mut ext = Extensions::new();
    ext.insert("Sample".to_string());
    ext.clear();
    assert!(ext.get::<String>().is_none());
}

