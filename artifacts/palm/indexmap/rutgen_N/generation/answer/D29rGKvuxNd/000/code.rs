// Answer 0

#[test]
fn test_first_mut_on_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    let result = map.first_mut();
    assert_eq!(result, None);
}

#[test]
fn test_first_mut_on_non_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 100);
    map.insert(2, 200);

    if let Some((key, value)) = map.first_mut() {
        assert_eq!(*key, 1);
        *value += 50; // Mutate the value
    }

    assert_eq!(map.get(&1), Some(&150));
}

#[test]
fn test_first_mut_with_multiple_entries() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(3, 300);
    map.insert(4, 400);
    map.insert(5, 500);

    if let Some((key, value)) = map.first_mut() {
        assert_eq!(*key, 3);
        *value *= 2; // Mutate the value
    }

    assert_eq!(map.get(&3), Some(&600));
}

