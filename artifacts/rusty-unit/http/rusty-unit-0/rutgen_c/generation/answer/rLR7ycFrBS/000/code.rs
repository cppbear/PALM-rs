// Answer 0

#[test]
fn test_get_when_empty() {
    let ext = Extensions::new();
    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_get_after_insert() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    assert_eq!(ext.get::<i32>(), Some(&5i32));
}

#[test]
fn test_get_for_different_type() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    assert!(ext.get::<f64>().is_none());
}

#[test]
fn test_get_multiple_types() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    ext.insert("Hello".to_string());
    
    assert_eq!(ext.get::<i32>(), Some(&5i32));
    assert_eq!(ext.get::<String>(), Some(&"Hello".to_string()));
    assert!(ext.get::<f64>().is_none());
}

