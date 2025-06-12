// Answer 0

#[test]
fn test_swap_remove_full_single_element() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};

    #[derive(Hash, PartialEq, Eq, Debug)]
    struct MyKey(usize);

    #[derive(Debug)]
    struct MyValue(String);

    let mut map: IndexMap<MyKey, MyValue, RandomState> = IndexMap::new();
    map.insert(MyKey(1), MyValue("value1".to_string()));

    let result = map.swap_remove_full(&MyKey(1));
    assert_eq!(result, Some((0, MyKey(1), MyValue("value1".to_string()))));
}

#[test]
fn test_swap_remove_full_multiple_elements() {
    use std::collections::hash_map::RandomState;

    #[derive(Hash, PartialEq, Eq, Debug)]
    struct MyKey(usize);

    #[derive(Debug)]
    struct MyValue(String);

    let mut map: IndexMap<MyKey, MyValue, RandomState> = IndexMap::new();
    map.insert(MyKey(1), MyValue("value1".to_string()));
    map.insert(MyKey(2), MyValue("value2".to_string()));

    let result = map.swap_remove_full(&MyKey(1));
    assert_eq!(result, Some((0, MyKey(1), MyValue("value1".to_string()))));
}

#[test]
fn test_swap_remove_full_not_found() {
    use std::collections::hash_map::RandomState;

    #[derive(Hash, PartialEq, Eq, Debug)]
    struct MyKey(usize);

    #[derive(Debug)]
    struct MyValue(String);

    let mut map: IndexMap<MyKey, MyValue, RandomState> = IndexMap::new();
    map.insert(MyKey(1), MyValue("value1".to_string()));
    map.insert(MyKey(2), MyValue("value2".to_string()));

    let result = map.swap_remove_full(&MyKey(3));
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_full_with_empty_map() {
    use std::collections::hash_map::RandomState;

    #[derive(Hash, PartialEq, Eq, Debug)]
    struct MyKey(usize);

    #[derive(Debug)]
    struct MyValue(String);

    let mut map: IndexMap<MyKey, MyValue, RandomState> = IndexMap::new();

    let result = map.swap_remove_full(&MyKey(1));
    assert_eq!(result, None);
}

