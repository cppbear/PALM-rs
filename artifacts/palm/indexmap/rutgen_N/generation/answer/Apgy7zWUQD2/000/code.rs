// Answer 0

#[test]
fn test_split_off_valid_case() {
    use indexmap::IndexMap;

    let mut set = IndexMap::new();
    set.insert(1, "a");
    set.insert(2, "b");
    set.insert(3, "c");

    let new_set = set.split_off(1);

    assert_eq!(new_set.len(), 2);
    assert_eq!(new_set.get(&2), Some(&"b"));
    assert_eq!(new_set.get(&3), Some(&"c"));
    assert_eq!(set.len(), 1);
    assert_eq!(set.get(&1), Some(&"a"));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_split_off_panic_case() {
    let mut set = indexmap::IndexMap::new();
    set.insert(1, "a");
    set.insert(2, "b");

    // This should panic because we are trying to split off at index 3, which is out of bounds.
    let _new_set = set.split_off(3);
}

#[test]
fn test_split_off_empty_set() {
    use indexmap::IndexMap;

    let mut set: IndexMap<i32, &str> = IndexMap::new();
    
    let new_set = set.split_off(0);

    assert_eq!(set.len(), 0);
    assert_eq!(new_set.len(), 0);
}

