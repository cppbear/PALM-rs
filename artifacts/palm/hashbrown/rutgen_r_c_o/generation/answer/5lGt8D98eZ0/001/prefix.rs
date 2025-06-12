// Answer 0

#[test]
fn test_shrink_to_fit_non_empty_map() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_empty_map() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_large_map() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(1000);
    for i in 0..500 {
        map.insert(i, i * 2);
    }
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_full_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(200);
    for i in 0..200 {
        map.insert(i, i * 2);
    }
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_with_removals() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(2, 3);
    map.remove(&1);
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_after_reserve() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(50);
    map.insert(1, 2);
    map.reserve(20);
    map.shrink_to_fit();
}

