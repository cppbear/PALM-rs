// Answer 0

#[test]
fn test_hashset_is_empty_initially() {
    let set: HashSet<i32> = HashSet::new();
    assert!(set.is_empty());
}

#[test]
fn test_hashset_is_empty_after_insert() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    assert!(!set.is_empty());
}

#[test]
fn test_hashset_is_empty_after_clear() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.clear();
    assert!(set.is_empty());
} 

#[test]
fn test_hashset_is_empty_after_draining() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    let _drained: Vec<_> = set.drain().collect();
    assert!(set.is_empty());
}

