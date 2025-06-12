// Answer 0

#[test]
fn test_insert_new_type() {
    let mut ext = Extensions::new();
    assert!(ext.insert(42i32).is_none());
}

#[test]
fn test_insert_different_type() {
    let mut ext = Extensions::new();
    assert!(ext.insert(13u8).is_none());
    assert!(ext.insert(3.14f64).is_none());
}

#[test]
fn test_replace_existing_type() {
    let mut ext = Extensions::new();
    assert!(ext.insert(99i32).is_none());
    assert_eq!(ext.insert(85i32), Some(99i32));
}

#[test]
fn test_insert_multiple_types() {
    let mut ext = Extensions::new();
    assert!(ext.insert(7i64).is_none());
    assert!(ext.insert("Hello".to_string()).is_none());
    assert_eq!(ext.insert(5i64), Some(7i64));
    assert_eq!(ext.insert("World".to_string()), Some("Hello".to_string()));
}

#[test]
fn test_insert_and_get() {
    let mut ext = Extensions::new();
    ext.insert(123i32);
    assert_eq!(ext.get::<i32>(), Some(&123));
}

#[test]
fn test_insert_and_get_mut() {
    let mut ext = Extensions::new();
    ext.insert(456i32);
    if let Some(val) = ext.get_mut::<i32>() {
        *val = 789;
    }
    assert_eq!(ext.get::<i32>(), Some(&789));
}

#[test]
fn test_clear() {
    let mut ext = Extensions::new();
    ext.insert(1i32);
    ext.insert(2u8);
    ext.clear();
    assert!(ext.is_empty());
}

#[test]
fn test_len() {
    let mut ext = Extensions::new();
    assert_eq!(ext.len(), 0);
    ext.insert(42i32);
    assert_eq!(ext.len(), 1);
    ext.insert(43u8);
    assert_eq!(ext.len(), 2);
    ext.clear();
    assert_eq!(ext.len(), 0);
}

#[test]
fn test_is_empty() {
    let mut ext = Extensions::new();
    assert!(ext.is_empty());
    ext.insert(1u32);
    assert!(!ext.is_empty());
    ext.clear();
    assert!(ext.is_empty());
}

