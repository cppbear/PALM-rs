// Answer 0

#[test]
fn test_remove_existing_value() {
    let mut ext = Extensions::new();
    ext.insert(10i32);
    let removed_value = ext.remove::<i32>();
    assert_eq!(removed_value, Some(10i32));
    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_remove_non_existing_value() {
    let mut ext = Extensions::new();
    ext.insert(20i32);
    let removed_value = ext.remove::<f64>();
    assert_eq!(removed_value, None);
    assert_eq!(ext.len(), 1);
}

#[test]
fn test_remove_before_insert() {
    let mut ext = Extensions::new();
    let removed_value = ext.remove::<i32>();
    assert_eq!(removed_value, None);
    assert!(ext.is_empty());
}

#[test]
fn test_remove_after_clear() {
    let mut ext = Extensions::new();
    ext.insert(30i32);
    ext.clear();
    let removed_value = ext.remove::<i32>();
    assert_eq!(removed_value, None);
}

#[test]
fn test_multiple_removals() {
    let mut ext = Extensions::new();
    ext.insert(40i32);
    ext.insert(50i32);

    let removed_value1 = ext.remove::<i32>();
    assert_eq!(removed_value1, Some(40i32));
    assert!(ext.get::<i32>().is_none());

    let removed_value2 = ext.remove::<i32>();
    assert_eq!(removed_value2, None);
}

