// Answer 0

#[test]
fn test_shift_remove_full_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    let result = map.shift_remove_full(&1);
    assert_eq!(result, None);
}

#[test]
fn test_shift_remove_full_single_element_not_present() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(2, 20);
    let result = map.shift_remove_full(&1);
    assert_eq!(result, None);
}

#[test]
fn test_shift_remove_full_single_element_present() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    let result = map.shift_remove_full(&1);
    assert_eq!(result, Some((0, 1, 10)));
}

