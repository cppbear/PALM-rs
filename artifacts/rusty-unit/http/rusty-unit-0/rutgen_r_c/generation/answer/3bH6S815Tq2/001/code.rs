// Answer 0

#[test]
fn test_is_empty_initially() {
    let ext = Extensions::new();
    assert!(ext.is_empty());
}

#[test]
fn test_is_empty_after_insertion() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    assert!(!ext.is_empty());
}

#[test]
fn test_is_empty_after_removal() {
    let mut ext = Extensions::new();
    ext.insert(10u32);
    ext.remove::<u32>();
    assert!(ext.is_empty());
}

#[test]
fn test_is_empty_after_clearing() {
    let mut ext = Extensions::new();
    ext.insert("test");
    ext.clear();
    assert!(ext.is_empty());
}

#[test]
fn test_is_empty_with_multiple_insertions() {
    let mut ext = Extensions::new();
    ext.insert(1i32);
    ext.insert(2i32);
    assert!(!ext.is_empty());
    ext.clear();
    assert!(ext.is_empty());
}

