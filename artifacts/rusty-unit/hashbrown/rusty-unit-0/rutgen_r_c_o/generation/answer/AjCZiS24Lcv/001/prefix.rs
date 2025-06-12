// Answer 0

#[test]
fn test_hashmap_new_empty() {
    let map: HashMap<&str, i32> = HashMap::new();
}

#[test]
fn test_hashmap_new_capacity_zero() {
    let map: HashMap<i32, i32> = HashMap::new();
}

#[test]
fn test_hashmap_new_empty_filled() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("key".to_string(), 1);
}

#[test]
fn test_hashmap_new_large() {
    let map: HashMap<usize, usize> = HashMap::new();
}

