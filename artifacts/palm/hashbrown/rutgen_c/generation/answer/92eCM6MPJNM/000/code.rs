// Answer 0

#[test]
fn test_into_keys() {
    use crate::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder, Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let mut vec: Vec<&str> = map.into_keys().collect();
    vec.sort_unstable();
    assert_eq!(vec, ["a", "b", "c"]);
}

#[test]
fn test_into_keys_empty() {
    use crate::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    let vec: Vec<&str> = map.into_keys().collect();
    assert!(vec.is_empty());
}

#[test]
fn test_into_keys_single_element() {
    use crate::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("only_key", 42);

    let mut vec: Vec<&str> = map.into_keys().collect();
    assert_eq!(vec, ["only_key"]);
}

