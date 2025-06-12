// Answer 0

#[test]
fn test_shrink_to_below_current_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(50);
}

#[test]
fn test_shrink_to_exact_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(100);
}

#[test]
fn test_shrink_to_zero() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(0);
}

#[test]
fn test_shrink_to_small_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(10);
}

#[test]
fn test_shrink_to_min_capacity_greater_than_current() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(20);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(30);
}

#[test]
fn test_shrink_to_empty_map() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.shrink_to(0);
}

#[test]
fn test_shrink_to_single_element() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.shrink_to(1);
}

