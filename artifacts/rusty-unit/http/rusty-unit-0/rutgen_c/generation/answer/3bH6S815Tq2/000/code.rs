// Answer 0

#[test]
fn test_is_empty_initially() {
    let ext = Extensions::new();
    assert!(ext.is_empty());
}

#[test]
fn test_is_empty_after_insert() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    assert!(!ext.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    ext.clear();
    assert!(ext.is_empty());
}

#[test]
fn test_is_empty_after_remove() {
    let mut ext = Extensions::new();
    ext.insert(10i32);
    ext.remove::<i32>();
    assert!(ext.is_empty());
}

