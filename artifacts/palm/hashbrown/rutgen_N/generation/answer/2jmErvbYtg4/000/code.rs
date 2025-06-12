// Answer 0

#[test]
fn test_clear_on_non_empty_set() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.clear();
    assert!(set.is_empty());
}

#[test]
fn test_clear_on_empty_set() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    set.clear();
    assert!(set.is_empty());
}

#[test]
fn test_clear_after_inserting_and_removing_elements() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.remove(&1);
    set.clear();
    assert!(set.is_empty());
}

