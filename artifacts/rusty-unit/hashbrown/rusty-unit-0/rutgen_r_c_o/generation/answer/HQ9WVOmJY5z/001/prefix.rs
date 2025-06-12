// Answer 0

#[test]
fn test_remove_existing_key() {
    let mut map = HashMap::new();
    map.insert(500, "value_500");
    let result = map.remove(&500);
}

#[test]
fn test_remove_another_existing_key() {
    let mut map = HashMap::new();
    map.insert(10, "value_10");
    let result = map.remove(&10);
}

#[test]
fn test_remove_middle_key() {
    let mut map = HashMap::new();
    map.insert(1000, "value_1000");
    let result = map.remove(&1000);
}

#[test]
fn test_remove_largest_key() {
    let mut map = HashMap::new();
    map.insert(1000000, "value_1000000");
    let result = map.remove(&1000000);
}

#[test]
fn test_remove_key_multiple_times() {
    let mut map = HashMap::new();
    map.insert(999, "value_999");
    let result_first = map.remove(&999);
    let result_second = map.remove(&999);
}

