// Answer 0

#[test]
fn test_partition_point_empty() {
    let entries: Vec<(i32, i32)> = Vec::new();
    let sorted_set = indexmap::IndexMap::<i32, i32>::from_iter(entries);
    
    let result = sorted_set.partition_point(|&key| key < 5);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_all_less() {
    let entries = vec![(1, 10), (2, 20), (3, 30)];
    let sorted_set = indexmap::IndexMap::<i32, i32>::from_iter(entries);
    
    let result = sorted_set.partition_point(|&key| key < 5);
    assert_eq!(result, 3);
}

#[test]
fn test_partition_point_all_greater() {
    let entries = vec![(5, 50), (6, 60), (7, 70)];
    let sorted_set = indexmap::IndexMap::<i32, i32>::from_iter(entries);
    
    let result = sorted_set.partition_point(|&key| key < 5);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_mixed() {
    let entries = vec![(1, 10), (3, 30), (5, 50), (7, 70)];
    let sorted_set = indexmap::IndexMap::<i32, i32>::from_iter(entries);
    
    let result = sorted_set.partition_point(|&key| key < 5);
    assert_eq!(result, 2);
}

#[test]
fn test_partition_point_boundary() {
    let entries = vec![(3, 30), (3, 40), (5, 50), (6, 60)];
    let sorted_set = indexmap::IndexMap::<i32, i32>::from_iter(entries);
    
    let result = sorted_set.partition_point(|&key| key <= 3);
    assert_eq!(result, 2);
}

#[test]
#[should_panic]
fn test_partition_point_panic() {
    let entries = vec![(1, 10), (3, 30), (2, 20)];
    let sorted_set = indexmap::IndexMap::<i32, i32>::from_iter(entries);
    
    // This should panic as the entries are not sorted
    let _ = sorted_set.partition_point(|&key| key < 5);
}

