// Answer 0

#[test]
fn test_get_present_key_1() {
    let mut map = IndexMap::new();
    map.insert(1, "value1");
    let result = map.get(&1);
}

#[test]
fn test_get_present_key_50() {
    let mut map = IndexMap::new();
    map.insert(50, "value50");
    let result = map.get(&50);
}

#[test]
fn test_get_present_key_100() {
    let mut map = IndexMap::new();
    map.insert(100, "value100");
    let result = map.get(&100);
}

#[test]
fn test_get_present_keys_multiple() {
    let mut map = IndexMap::new();
    for i in 1..=100 {
        map.insert(i, format!("value{}", i));
    }
    let result_1 = map.get(&1);
    let result_50 = map.get(&50);
    let result_100 = map.get(&100);
}

