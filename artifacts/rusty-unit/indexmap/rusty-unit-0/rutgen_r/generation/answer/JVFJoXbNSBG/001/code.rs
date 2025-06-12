// Answer 0

#[test]
fn test_split_off_valid_case() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(0, 0);
    map.insert(1, 1);
    map.insert(2, 2);

    let split_map = map.split_off(2);

    assert_eq!(split_map.len(), 1);
    assert_eq!(split_map.get(&2), Some(&2));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&0), Some(&0));
    assert_eq!(map.get(&1), Some(&1));
}

#[test]
#[should_panic]
fn test_split_off_panic_condition() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(0, 0);
    map.insert(1, 1);
    
    // This should panic as the index is greater than the length of the map (2).
    let _split_map = map.split_off(3);
}

#[test]
fn test_split_off_boundary_case() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(0, 0);

    let split_map = map.split_off(1);

    assert_eq!(split_map.len(), 0);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&0), Some(&0));
}

#[test]
fn test_split_off_empty_map() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();

    let split_map = map.split_off(0);

    assert_eq!(split_map.len(), 0);
    assert_eq!(map.len(), 0);
}

