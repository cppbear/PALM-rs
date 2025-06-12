// Answer 0

#[test]
fn test_contains_with_existing_value() {
    use hashbrown::HashSet;
    use std::collections::hash_map::RandomState;

    let mut set: HashSet<i32, RandomState> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert_eq!(set.contains(&1), true);
}

#[test]
fn test_contains_with_non_existing_value() {
    use hashbrown::HashSet;
    use std::collections::hash_map::RandomState;

    let mut set: HashSet<i32, RandomState> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert_eq!(set.contains(&4), false);
}

#[test]
fn test_contains_with_reference() {
    use hashbrown::HashSet;
    use std::collections::hash_map::RandomState;

    let set: HashSet<i32, RandomState> = [1, 2, 3].into_iter().collect();

    let value: &i32 = &2;
    assert_eq!(set.contains(value), true);
} 

#[test]
fn test_contains_with_borrowed_integer() {
    use hashbrown::HashSet;
    use std::collections::hash_map::RandomState;

    let set: HashSet<i32, RandomState> = [10, 20, 30].into_iter().collect();
    let value: i32 = 20;

    assert_eq!(set.contains(&value), true);
} 

#[test]
fn test_contains_with_empty_set() {
    use hashbrown::HashSet;
    use std::collections::hash_map::RandomState;

    let set: HashSet<i32, RandomState> = HashSet::new();

    assert_eq!(set.contains(&1), false);
} 

#[test]
fn test_contains_with_negative_value() {
    use hashbrown::HashSet;
    use std::collections::hash_map::RandomState;

    let mut set: HashSet<i32, RandomState> = HashSet::new();
    set.insert(-1);
    set.insert(-2);
    set.insert(-3);

    assert_eq!(set.contains(&-2), true);
} 

#[test]
fn test_contains_with_large_set() {
    use hashbrown::HashSet;
    use std::collections::hash_map::RandomState;

    let mut set: HashSet<i32, RandomState> = HashSet::new();
    for i in 0..1000 {
        set.insert(i);
    }

    assert_eq!(set.contains(&500), true);
    assert_eq!(set.contains(&1000), false);
}

