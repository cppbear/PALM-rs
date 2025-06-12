// Answer 0

#[test]
fn test_remove_existing_value() {
    use indexmap::IndexSet;
    use std::hash::Hash;

    let mut set: IndexSet<i32> = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert!(set.remove(&2));
    assert!(!set.contains(&2));
    assert_eq!(set.len(), 2);
}

#[test]
fn test_remove_non_existing_value() {
    use indexmap::IndexSet;

    let mut set: IndexSet<i32> = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert!(!set.remove(&4));
    assert_eq!(set.len(), 3);
}

#[test]
fn test_remove_value_from_empty_set() {
    use indexmap::IndexSet;

    let mut set: IndexSet<i32> = IndexSet::new();

    assert!(!set.remove(&1));
    assert_eq!(set.len(), 0);
}

#[test]
fn test_remove_last_element() {
    use indexmap::IndexSet;

    let mut set: IndexSet<i32> = IndexSet::new();
    set.insert(1);

    assert!(set.remove(&1));
    assert_eq!(set.len(), 0);
}

