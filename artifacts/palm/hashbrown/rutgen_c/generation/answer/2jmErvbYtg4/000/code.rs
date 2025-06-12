// Answer 0

#[test]
fn test_clear_removes_all_values() {
    let mut set = HashSet::<i32>::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.clear();
    assert!(set.is_empty());
}

#[test]
fn test_clear_on_empty_set() {
    let mut set = HashSet::<i32>::new();
    set.clear();
    assert!(set.is_empty());
}

#[test]
fn test_clear_after_insertions() {
    let mut set = HashSet::<i32>::new();
    set.insert(10);
    set.insert(20);
    set.insert(30);
    assert_eq!(set.len(), 3);
    set.clear();
    assert_eq!(set.len(), 0);
}

