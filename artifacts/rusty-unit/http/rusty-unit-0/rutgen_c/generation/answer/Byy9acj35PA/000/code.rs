// Answer 0

#[test]
fn test_clear_empty_extensions() {
    let mut ext = Extensions::new();
    ext.clear();
    assert!(ext.is_empty());
}

#[test]
fn test_clear_non_empty_extensions() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    ext.insert("example".to_string());
    ext.clear();
    assert!(ext.get::<i32>().is_none());
    assert!(ext.get::<String>().is_none());
    assert!(ext.is_empty());
}

#[test]
fn test_clear_after_insertion_and_removal() {
    let mut ext = Extensions::new();
    ext.insert(10u32);
    ext.insert(vec![1, 2, 3]);
    ext.clear();
    assert!(ext.get::<u32>().is_none());
    assert!(ext.get::<Vec<i32>>().is_none());
    assert_eq!(ext.len(), 0);
}

