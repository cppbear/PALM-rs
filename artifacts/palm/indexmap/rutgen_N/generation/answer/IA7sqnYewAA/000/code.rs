// Answer 0

#[test]
fn test_partition_point_no_elements() {
    let map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    let pred = |&key: &i32, &value: &i32| key < 5;
    assert_eq!(map.partition_point(pred), 0);
}

#[test]
fn test_partition_point_all_elements_partitioned() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let pred = |&key: &i32, &value: &i32| key > 3;
    assert_eq!(map.partition_point(pred), 3);
}

#[test]
fn test_partition_point_some_elements_partitioned() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let pred = |&key: &i32, &value: &i32| key < 3;
    assert_eq!(map.partition_point(pred), 2);
}

#[test]
fn test_partition_point_boundary_condition() {
    let mut map = indexmap::IndexMap::new();
    map.insert(5, 50);
    map.insert(10, 100);
    
    let pred = |&key: &i32, &value: &i32| key <= 5;
    assert_eq!(map.partition_point(pred), 1);
}

#[test]
fn test_partition_point_on_sorted_map() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);
    
    let pred = |&key: &i32, &value: &i32| key < 3;
    assert_eq!(map.partition_point(pred), 2);
}

