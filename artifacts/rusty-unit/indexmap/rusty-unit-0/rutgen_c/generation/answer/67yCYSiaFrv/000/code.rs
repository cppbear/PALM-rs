// Answer 0

#[test]
fn test_get_full_existing_key() {
    #[derive(Hash, Eq, PartialEq)]
    struct MyKey(usize);

    #[derive(Debug)]
    struct MyValue(String);

    let mut index_map = IndexMap::<MyKey, MyValue, RandomState>::new();
    index_map.insert(MyKey(1), MyValue("Value1".to_string()));

    let result = index_map.get_full(&MyKey(1));
    assert_eq!(result, Some((0, &MyKey(1), &MyValue("Value1".to_string()))));
}

#[test]
fn test_get_full_non_existing_key() {
    #[derive(Hash, Eq, PartialEq)]
    struct MyKey(usize);

    #[derive(Debug)]
    struct MyValue(String);

    let mut index_map = IndexMap::<MyKey, MyValue, RandomState>::new();
    index_map.insert(MyKey(1), MyValue("Value1".to_string()));

    let result = index_map.get_full(&MyKey(2));
    assert_eq!(result, None);
}

#[test]
fn test_get_full_empty_index_map() {
    #[derive(Hash, Eq, PartialEq)]
    struct MyKey(usize);

    #[derive(Debug)]
    struct MyValue(String);

    let mut index_map = IndexMap::<MyKey, MyValue, RandomState>::new();

    let result = index_map.get_full(&MyKey(1));
    assert_eq!(result, None);
}

