// Answer 0

#[test]
fn test_shrink_to_with_zero_capacity() {
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.shrink_to(0);
}

#[test]
fn test_shrink_to_with_min_capacity_one() {
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.shrink_to(1);
}

#[test]
fn test_shrink_to_with_min_capacity_two() {
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.shrink_to(2);
}

#[test]
fn test_shrink_to_with_min_capacity_three() {
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.shrink_to(3);
}

#[test]
fn test_shrink_to_with_min_capacity_ten() {
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.shrink_to(10);
}

#[test]
fn test_shrink_to_with_min_capacity_one_hundred() {
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.shrink_to(100);
}

#[should_panic]
fn test_shrink_to_with_min_capacity_exceeding_current() {
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.shrink_to(101);
}

#[should_panic]
fn test_shrink_to_with_min_capacity_exceeding_current_empty() {
    let mut set = HashSet::with_capacity(0);
    set.shrink_to(1);
}

#[test]
fn test_shrink_to_with_min_capacity_max() {
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.shrink_to(usize::MAX);
}

