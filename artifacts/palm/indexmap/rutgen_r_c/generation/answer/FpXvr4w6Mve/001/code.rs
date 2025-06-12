// Answer 0

#[test]
fn test_partition_point_empty() {
    let set: super::IndexSet<i32, ()> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: () } };
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_single_element() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: () } };
    set.push(10);
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_first_partition() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: () } };
    set.push(2);
    set.push(4);
    set.push(6);
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 2);
}

#[test]
fn test_partition_point_last_partition() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: () } };
    set.push(1);
    set.push(2);
    set.push(3);
    let result = set.partition_point(|&x| x < 1);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_all_elements_partitioned() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: () } };
    set.push(1);
    set.push(3);
    set.push(5);
    let result = set.partition_point(|&x| x < 0);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_non_partitioned() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet { map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: () } };
    set.push(2);
    set.push(4);
    set.push(6);
    let result = set.partition_point(|&x| x > 10);
    assert_eq!(result, 3);
}

