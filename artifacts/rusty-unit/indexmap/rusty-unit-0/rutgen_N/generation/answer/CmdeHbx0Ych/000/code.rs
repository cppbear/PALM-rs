// Answer 0

#[test]
fn test_with_hasher_creates_set() {
    use indexmap::{IndexMap, IndexSet};
    use std::collections::hash_map::RandomState;

    struct CustomHasher {
        hasher: RandomState,
    }

    let hasher = CustomHasher { hasher: RandomState::new() };
    let set: IndexSet<i32> = IndexSet::with_hasher(hasher.hasher.clone());

    assert!(set.is_empty());
}

#[test]
fn test_with_hasher_non_empty_set() {
    use indexmap::{IndexMap, IndexSet};
    use std::collections::hash_map::RandomState;

    struct CustomHasher {
        hasher: RandomState,
    }

    let hasher = CustomHasher { hasher: RandomState::new() };
    let mut set: IndexSet<i32> = IndexSet::with_hasher(hasher.hasher.clone());

    set.insert(1);
    set.insert(2);

    assert_eq!(set.len(), 2);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
}

