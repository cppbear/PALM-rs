// Answer 0

#[test]
fn test_length_initially_zero() {
    let ext = Extensions::new();
    assert_eq!(ext.len(), 0);
}

#[test]
fn test_length_after_insertion() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    assert_eq!(ext.len(), 1);
}

#[test]
fn test_length_after_multiple_insertions() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    ext.insert("Hello".to_string());
    ext.insert(vec![1, 2, 3]);
    assert_eq!(ext.len(), 3);
}

#[test]
fn test_length_after_clearing() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    ext.clear();
    assert_eq!(ext.len(), 0);
}

