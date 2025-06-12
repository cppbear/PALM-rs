// Answer 0

#[test]
fn test_is_disjoint_when_self_is_larger() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::new();
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::new();

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    set_b.insert(4);
    set_b.insert(5);

    let result = set_a.is_disjoint(&set_b);
    assert!(result); // expect disjoint since there are no common elements
}

#[test]
fn test_is_disjoint_when_self_contains_some_elements() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::new();
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::new();

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    set_b.insert(2);
    set_b.insert(4);
    set_b.insert(5);

    let result = set_a.is_disjoint(&set_b);
    assert!(!result); // expect not disjoint since 2 is common
}

#[test]
fn test_is_disjoint_when_self_filled_with_various_elements() {
    use indexmap::IndexSet;
    use std::collections::hash_map::RandomState;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::new();
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::new();

    set_a.insert(10);
    set_a.insert(20);
    set_a.insert(30);
    set_a.insert(40);

    set_b.insert(50);
    set_b.insert(60);
    set_b.insert(70);
    set_b.insert(80);

    let result = set_a.is_disjoint(&set_b);
    assert!(result); // expect disjoint since there are no common elements
}

