// Answer 0

#[test]
fn test_is_empty_new_extensions() {
    let ext = Extensions::new();
    ext.is_empty();
}

#[test]
fn test_is_empty_after_insert_zero() {
    let mut ext = Extensions::new();
    ext.insert(0i32);
    ext.is_empty();
}

#[test]
fn test_is_empty_after_insert_positive() {
    let mut ext = Extensions::new();
    ext.insert(10i32);
    ext.is_empty();
}

#[test]
fn test_is_empty_after_multiple_inserts() {
    let mut ext = Extensions::new();
    ext.insert(10i32);
    ext.insert(20i32);
    ext.insert(30i32);
    ext.is_empty();
}

#[test]
fn test_is_empty_after_clear() {
    let mut ext = Extensions::new();
    ext.insert(42i32);
    ext.clear();
    ext.is_empty();
}

#[test]
fn test_is_empty_multiple_clears() {
    let mut ext = Extensions::new();
    ext.insert(100i32);
    ext.clear();
    ext.clear();
    ext.is_empty();
}

#[test]
fn test_is_empty_with_large_insertions() {
    let mut ext = Extensions::new();
    for i in 0..10_000 {
        ext.insert(i);
    }
    ext.is_empty();
}

#[test]
fn test_is_empty_after_removal() {
    let mut ext = Extensions::new();
    ext.insert(5i32);
    ext.remove::<i32>();
    ext.is_empty();
}

#[test]
fn test_is_empty_with_default_insert() {
    let mut ext = Extensions::new();
    ext.get_or_insert_default::<String>();
    ext.is_empty();
}

