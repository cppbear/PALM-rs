// Answer 0

#[test]
fn test_is_superset_with_empty_other() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let other = IndexSet::<_, RandomState>::new(); // empty set

    assert!(set.is_superset(&other));
}

#[test]
fn test_is_superset_with_equal_sets() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let mut other = IndexSet::new();
    other.insert(1);
    other.insert(2);
    other.insert(3);

    assert!(set.is_superset(&other));
}

#[test]
fn test_is_superset_with_non_subset() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let mut other = IndexSet::new();
    other.insert(4);
    other.insert(5);
    
    assert!(!set.is_superset(&other));
}

#[test]
fn test_is_superset_with_partial_overlap() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let mut other = IndexSet::new();
    other.insert(2);
    other.insert(3);

    assert!(set.is_superset(&other));
}

#[test]
fn test_is_superset_with_identical_elements_different_order() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);

    let mut other = IndexSet::new();
    other.insert(1);
    other.insert(2);
    other.insert(3);

    assert!(set.is_superset(&other));
}

