// Answer 0

#[test]
fn test_is_superset_with_disjoint_sets() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::new();
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::new();

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    set_b.insert(4);
    set_b.insert(5);

    assert!(!set_a.is_superset(&set_b));
}

#[test]
fn test_is_superset_with_subset() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::new();
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::new();

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    set_b.insert(2);
    set_b.insert(3);

    assert!(set_a.is_superset(&set_b));
}

#[test]
fn test_is_superset_with_equal_sets() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::new();
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::new();

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    set_b.insert(1);
    set_b.insert(2);
    set_b.insert(3);

    assert!(set_a.is_superset(&set_b));
}

#[test]
fn test_is_superset_with_empty_other() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::new();
    set_a.insert(1);
    
    let set_b: IndexSet<i32, RandomState> = IndexSet::new();

    assert!(set_a.is_superset(&set_b));
}

#[test]
fn test_is_superset_with_empty_self() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexSet;

    let set_a: IndexSet<i32, RandomState> = IndexSet::new();
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::new();

    set_b.insert(1);
    
    assert!(!set_a.is_superset(&set_b));
}

