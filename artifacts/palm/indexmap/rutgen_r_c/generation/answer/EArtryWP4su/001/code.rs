// Answer 0

#[test]
fn test_sub_different_sets() {
    use std::collections::hash_map::RandomState;
    use crate::{IndexSet};

    let mut set_a = IndexSet::<i32, RandomState>::new();
    let mut set_b = IndexSet::<i32, RandomState>::new();

    set_a.extend(vec![1, 2, 3, 4].into_iter());
    set_b.extend(vec![3, 4, 5].into_iter());

    let result = &set_a - &set_b;

    let expected: Vec<i32> = vec![1, 2];
    let result_vec: Vec<i32> = result.iter().cloned().collect();

    assert_eq!(result_vec, expected);
}

#[test]
fn test_sub_empty_both_sets() {
    use std::collections::hash_map::RandomState;
    use crate::{IndexSet};

    let set_a = IndexSet::<i32, RandomState>::new();
    let set_b = IndexSet::<i32, RandomState>::new();

    let result = &set_a - &set_b;

    assert!(result.is_empty());
}

#[test]
fn test_sub_empty_first_set() {
    use std::collections::hash_map::RandomState;
    use crate::{IndexSet};

    let set_a = IndexSet::<i32, RandomState>::new();
    let mut set_b = IndexSet::<i32, RandomState>::new();

    set_b.extend(vec![1, 2, 3].into_iter());

    let result = &set_a - &set_b;

    assert!(result.is_empty());
}

#[test]
fn test_sub_empty_second_set() {
    use std::collections::hash_map::RandomState;
    use crate::{IndexSet};

    let mut set_a = IndexSet::<i32, RandomState>::new();
    set_a.extend(vec![1, 2, 3].into_iter());
    let set_b = IndexSet::<i32, RandomState>::new();

    let result = &set_a - &set_b;

    let expected: Vec<i32> = vec![1, 2, 3];
    let result_vec: Vec<i32> = result.iter().cloned().collect();

    assert_eq!(result_vec, expected);
}

#[test]
fn test_sub_identical_sets() {
    use std::collections::hash_map::RandomState;
    use crate::{IndexSet};

    let mut set_a = IndexSet::<i32, RandomState>::new();
    let mut set_b = IndexSet::<i32, RandomState>::new();

    set_a.extend(vec![1, 2, 3].into_iter());
    set_b.extend(vec![1, 2, 3].into_iter());

    let result = &set_a - &set_b;

    assert!(result.is_empty());
}

