// Answer 0

#[test]
fn test_shrink_to_zero_capacity() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(10);
    map.shrink_to(0);
}

#[test]
fn test_shrink_to_current_capacity() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(10);
    map.reserve(5);
    map.shrink_to(5);
}

#[test]
fn test_shrink_to_max_capacity() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    map.shrink_to(IndexMapCore::MAX_ENTRIES_CAPACITY);
}

#[test]
fn test_shrink_to_half_capacity() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(10);
    map.reserve(5);
    map.shrink_to(2);
}

#[test]
#[should_panic]
fn test_shrink_to_exceed_capacity() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(5);
    map.shrink_to(6);
}

#[test]
fn test_shrink_to_min_capacity_one() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(10);
    map.reserve(5);
    map.shrink_to(1);
}

#[test]
fn test_shrink_to_min_capacity_same_size() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(10);
    map.reserve(5);
    map.shrink_to(5);
}

#[test]
fn test_shrink_to_min_capacity_full() {
    let mut map = IndexMapCore::<usize, usize>::new();
    map.shrink_to(0);
}

