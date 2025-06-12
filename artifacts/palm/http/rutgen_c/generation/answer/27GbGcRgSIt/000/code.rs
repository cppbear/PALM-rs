// Answer 0

#[test]
fn test_insert_new_value() {
    let mut ext = Extensions::new();
    assert!(ext.insert(5i32).is_none());
}

#[test]
fn test_insert_different_type() {
    let mut ext = Extensions::new();
    assert!(ext.insert(5i32).is_none());
    assert!(ext.insert(4u8).is_none());
}

#[test]
fn test_insert_replace_existing_value() {
    let mut ext = Extensions::new();
    assert!(ext.insert(5i32).is_none());
    assert_eq!(ext.insert(9i32), Some(5i32));
}

#[test]
fn test_insert_with_same_type() {
    let mut ext = Extensions::new();
    assert!(ext.insert(10u64).is_none());
    assert_eq!(ext.insert(20u64), Some(10u64));
}

