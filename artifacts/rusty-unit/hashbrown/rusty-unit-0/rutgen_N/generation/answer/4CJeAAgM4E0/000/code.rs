// Answer 0

#[test]
fn test_is_empty_on_new_hashset() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = HashSet::new();
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_after_insert() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    assert!(!set.is_empty());
}

#[test]
fn test_is_empty_after_removal() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.remove(&1);
    assert!(set.is_empty());
}

