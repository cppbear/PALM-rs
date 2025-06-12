// Answer 0

#[test]
fn test_shrink_to_zero_capacity() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    set.shrink_to(0);
}

#[test]
fn test_shrink_to_exact_capacity() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    set.reserve(5);
    set.shrink_to(5);
}

#[test]
fn test_shrink_to_non_empty_set() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    set.reserve(5);
    set.shrink_to(2);
}

#[test]
fn test_shrink_to_min_capacity_less_than_current() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    set.reserve(5);
    set.shrink_to(3);
}

#[test]
#[should_panic]
fn test_shrink_to_invalid_capacity() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    set.shrink_to(15); // Invalid since 15 > current capacity
}

#[test]
fn test_shrink_to_capacity_equal_to_zero() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    set.reserve(3);
    set.shrink_to(0);
}

