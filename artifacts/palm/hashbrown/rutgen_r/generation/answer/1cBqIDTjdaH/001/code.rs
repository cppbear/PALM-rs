// Answer 0

#[test]
fn test_into_values_with_non_empty_map() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("x", 10);
    map.insert("y", 20);
    map.insert("z", 30);

    let mut vec: Vec<i32> = map.into_values().collect();
    vec.sort_unstable();
    assert_eq!(vec, [10, 20, 30]);
}

#[test]
fn test_into_values_with_empty_map() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    let vec: Vec<i32> = map.into_values().collect();
    assert_eq!(vec, Vec::<i32>::new());
}

#[test]
fn test_into_values_with_single_entry_map() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 42);

    let mut vec: Vec<i32> = map.into_values().collect();
    vec.sort_unstable();
    assert_eq!(vec, [42]);
}

#[test]
fn test_into_values_with_repeated_inserts() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("key1", 100);
    map.insert("key2", 200);
    map.insert("key2", 300); // Update the same key

    let mut vec: Vec<i32> = map.into_values().collect();
    vec.sort_unstable();
    assert_eq!(vec, [100, 300]);
}

#[test]
fn test_into_values_order_not_important() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    let mut vec: Vec<i32> = map.into_values().collect();
    vec.sort_unstable();
    assert_eq!(vec, [1, 2, 3]);
}

