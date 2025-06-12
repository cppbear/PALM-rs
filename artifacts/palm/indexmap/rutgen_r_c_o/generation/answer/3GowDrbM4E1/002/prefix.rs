// Answer 0

#[test]
fn test_is_subset_self_longer_than_other() {
    use std::collections::hash_map::RandomState;
    use crate::IndexSet;

    let hasher = RandomState::new();
    let mut other = IndexSet::with_capacity_and_hasher(5, hasher.clone());
    other.reserve(5);
    // Adding elements to `other`
    for i in 1..=5 {
        other.insert(i);
    }

    let mut self_set = IndexSet::with_capacity_and_hasher(10, hasher);
    self_set.reserve(10);
    // Adding more elements to `self` than `other`
    for i in 1..=6 {
        self_set.insert(i);
    }
    
    let result = self_set.is_subset(&other);
}

#[test]
fn test_is_subset_self_and_other_equal_length_different_elements() {
    use std::collections::hash_map::RandomState;
    use crate::IndexSet;

    let hasher = RandomState::new();
    let mut other = IndexSet::with_capacity_and_hasher(5, hasher.clone());
    other.reserve(5);
    // Adding different elements to `other`
    for i in 6..=10 {
        other.insert(i);
    }

    let mut self_set = IndexSet::with_capacity_and_hasher(5, hasher);
    self_set.reserve(5);
    // Adding equal length but different elements
    for i in 1..=5 {
        self_set.insert(i);
    }
    
    let result = self_set.is_subset(&other);
}

#[test]
fn test_is_subset_self_empty_other_non_empty() {
    use std::collections::hash_map::RandomState;
    use crate::IndexSet;

    let hasher = RandomState::new();
    let mut other = IndexSet::with_capacity_and_hasher(5, hasher.clone());
    other.reserve(5);
    // Adding elements to `other`
    for i in 1..=3 {
        other.insert(i);
    }

    let self_set = IndexSet::with_capacity_and_hasher(0, hasher);
    // `self_set` is empty, so should return true
    let result = self_set.is_subset(&other);
}

#[test]
fn test_is_subset_self_non_empty_other_empty() {
    use std::collections::hash_map::RandomState;
    use crate::IndexSet;

    let hasher = RandomState::new();
    let other = IndexSet::with_capacity_and_hasher(0, hasher.clone());

    let mut self_set = IndexSet::with_capacity_and_hasher(3, hasher);
    self_set.reserve(3);
    // Adding elements to `self`
    for i in 1..=3 {
        self_set.insert(i);
    }

    // `self_set` is non-empty and `other` is empty, should return false
    let result = self_set.is_subset(&other);
}

#[test]
fn test_is_subset_self_with_elements_in_other() {
    use std::collections::hash_map::RandomState;
    use crate::IndexSet;

    let hasher = RandomState::new();
    let mut other = IndexSet::with_capacity_and_hasher(5, hasher.clone());
    other.reserve(5);
    // Adding elements to `other`
    for i in 1..=5 {
        other.insert(i);
    }

    let mut self_set = IndexSet::with_capacity_and_hasher(3, hasher);
    self_set.reserve(3);
    // Elements added to `self` that are all contained in `other`
    self_set.insert(1);
    self_set.insert(2);
    self_set.insert(3);
    
    let result = self_set.is_subset(&other);
}

