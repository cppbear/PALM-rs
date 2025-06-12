// Answer 0

#[test]
fn test_remove_existing_type() {
    let mut ext = Extensions::new();
    ext.insert(10i32);
    let removed_value = ext.remove::<i32>();
    assert_eq!(removed_value, Some(10i32));
    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_remove_non_existing_type() {
    let mut ext = Extensions::new();
    ext.insert(20i32);
    let removed_value: Option<f64> = ext.remove();
    assert_eq!(removed_value, None);
}

#[test]
fn test_remove_empty_extensions() {
    let mut ext = Extensions::new();
    let removed_value: Option<i32> = ext.remove();
    assert_eq!(removed_value, None);
}

#[test]
fn test_remove_multiple_types() {
    let mut ext = Extensions::new();
    ext.insert("Hello".to_string());
    ext.insert(42i32);
    
    let removed_string = ext.remove::<String>();
    assert_eq!(removed_string, Some("Hello".to_string()));
    
    let removed_int = ext.remove::<i32>();
    assert_eq!(removed_int, Some(42i32));
    
    assert!(ext.get::<String>().is_none());
    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_remove_after_inserting_different_types() {
    let mut ext = Extensions::new();
    ext.insert(3.14f64);
    ext.insert("Test".to_string());

    let removed_float = ext.remove::<f64>();
    assert_eq!(removed_float, Some(3.14f64));
    assert!(ext.get::<f64>().is_none());

    let removed_string = ext.remove::<String>();
    assert_eq!(removed_string, Some("Test".to_string()));
    assert!(ext.get::<String>().is_none());
}

