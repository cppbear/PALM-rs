// Answer 0

#[test]
fn test_partition_point_all_elements_greater() {
    let map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::from_iter(vec![(1, 10), (2, 20), (3, 30)]);
    let index = map.partition_point(|key, value| *key > 3);
    assert_eq!(index, 3);
}

#[test]
fn test_partition_point_all_elements_less() {
    let map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::from_iter(vec![(1, 10), (2, 20), (3, 30)]);
    let index = map.partition_point(|key, value| *key < 1);
    assert_eq!(index, 0);
}

#[test]
fn test_partition_point_mixed_elements() {
    let map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::from_iter(vec![(1, 10), (2, 20), (3, 30)]);
    let index = map.partition_point(|key, value| *key < 2);
    assert_eq!(index, 1);
}

#[test]
fn test_partition_point_empty_map() {
    let map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    let index = map.partition_point(|key, value| *key < 1);
    assert_eq!(index, 0);
}

#[test]
#[should_panic]
fn test_partition_point_panic_condition() {
    let map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::from_iter(vec![(1, 10)]);
    let _index = map.partition_point(|k, _| panic!("This should trigger a panic."));
}

