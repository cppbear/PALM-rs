// Answer 0

#[test]
fn test_first_mut_with_non_empty_map() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let result = map.first_mut();
}

#[test]
fn test_first_mut_with_single_element_map() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    map.insert(1, 10);
    let result = map.first_mut();
}

#[test]
fn test_first_mut_with_empty_map() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    let result = map.first_mut();
}

#[test]
fn test_first_mut_with_negative_keys() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    map.insert(-1, 10);
    map.insert(-2, 20);
    let result = map.first_mut();
}

#[test]
fn test_first_mut_with_large_values() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    map.insert(1, 1_000_000);
    map.insert(2, 999_999);
    let result = map.first_mut();
}

#[test]
fn test_first_mut_with_duplicate_keys() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    map.insert(1, 10);
    map.insert(1, 20);
    let result = map.first_mut();
}

#[test]
fn test_first_mut_with_maximum_capacity() {
    let mut map = IndexMap::<i32, i32, RandomState>::new();
    for i in 0..1_000_000 {
        map.insert(i, i * 10);
    }
    let result = map.first_mut();
}

