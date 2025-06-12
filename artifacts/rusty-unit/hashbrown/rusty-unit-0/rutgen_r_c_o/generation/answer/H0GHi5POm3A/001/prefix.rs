// Answer 0

#[test]
fn test_get_existing_key() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let result = map.get(&1);
    let result2 = map.get(&2);
    let result3 = map.get(&3);
}

#[test]
fn test_get_another_existing_key() {
    let mut map = HashMap::new();
    map.insert(10, "x");
    map.insert(20, "y");
    map.insert(30, "z");

    let result = map.get(&20);
}

#[test]
fn test_get_with_large_keys() {
    let mut map = HashMap::new();
    for i in 1..=1000 {
        map.insert(i, i * 10);
    }

    let result = map.get(&(1000));
}

#[test]
fn test_get_existing_keys_with_different_values() {
    let mut map = HashMap::new();
    map.insert(50, "apple");
    map.insert(60, "banana");

    let result = map.get(&50);
    let result2 = map.get(&60);
}

#[test]
fn test_get_multiple_existing_keys() {
    let mut map = HashMap::new();
    map.insert(5, "alpha");
    map.insert(6, "beta");
    map.insert(7, "gamma");

    let result = map.get(&5);
    let result2 = map.get(&6);
    let result3 = map.get(&7);
}

