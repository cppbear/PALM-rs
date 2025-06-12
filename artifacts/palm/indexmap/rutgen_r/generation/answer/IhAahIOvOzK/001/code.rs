// Answer 0

#[test]
fn test_last_with_non_empty_indexmap() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("first", 1);
    map.insert("second", 2);
    map.insert("third", 3);

    assert_eq!(map.last(), Some(&3));
}

#[test]
fn test_last_with_single_element_indexmap() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("only", 100);

    assert_eq!(map.last(), Some(&100));
}

#[test]
fn test_last_with_empty_indexmap() {
    use indexmap::IndexMap;

    let map: IndexMap<&str, i32> = IndexMap::new();

    assert_eq!(map.last(), None);
}

#[test]
#[should_panic]
fn test_last_after_removal() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("first", 1);
    map.insert("second", 2);
    map.remove("second");

    // Check the last element after removal
    assert_eq!(map.last(), Some(&1)); // This should not panic but checks the condition
}

