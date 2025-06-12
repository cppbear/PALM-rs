// Answer 0

#[test]
fn test_insert_sorted_new_key() {
    let mut map = IndexMap::new();
    let key = 50;
    let value = 25;
    map.insert_sorted(key, value);
}

#[test]
fn test_insert_sorted_at_start() {
    let mut map = IndexMap::new();
    let key1 = 100;
    let value1 = 10;
    let key2 = 200;
    let value2 = 20;
    map.insert_sorted(key1, value1);
    map.insert_sorted(key2, value2);
    let new_key = 50;
    let new_value = 15;
    map.insert_sorted(new_key, new_value);
}

#[test]
fn test_insert_sorted_at_end() {
    let mut map = IndexMap::new();
    let key1 = 100;
    let value1 = 10;
    let key2 = 200;
    let value2 = 20;
    map.insert_sorted(key1, value1);
    map.insert_sorted(key2, value2);
    let new_key = 250;
    let new_value = 30;
    map.insert_sorted(new_key, new_value);
}

#[test]
fn test_insert_sorted_between_existing_keys() {
    let mut map = IndexMap::new();
    let key1 = 50;
    let value1 = 5;
    let key2 = 150;
    let value2 = 15;
    map.insert_sorted(key1, value1);
    map.insert_sorted(key2, value2);
    let new_key = 100;
    let new_value = 10;
    map.insert_sorted(new_key, new_value);
}

#[test]
fn test_insert_sorted_duplicate_key() {
    let mut map = IndexMap::new();
    let key = 75;
    let value1 = 5;
    let value2 = 10;
    map.insert_sorted(key, value1);
    let (_, old_value) = map.insert_sorted(key, value2);
}

#[test]
fn test_insert_sorted_edge_case_empty() {
    let mut map = IndexMap::new();
    let key = 0;
    let value = 1;
    map.insert_sorted(key, value);
}

#[test]
fn test_insert_sorted_large_values() {
    let mut map = IndexMap::new();
    for i in (0..1000).step_by(10) {
        map.insert_sorted(i, i + 5);
    }
    let key = 500;
    let value = 600;
    map.insert_sorted(key, value);
}

