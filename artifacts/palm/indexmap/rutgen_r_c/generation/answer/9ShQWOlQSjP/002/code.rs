// Answer 0

#[test]
fn test_get_range_mut_full_range() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: Default::default(), key: 1, value: "one" },
        Bucket { hash: Default::default(), key: 2, value: "two" },
        Bucket { hash: Default::default(), key: 3, value: "three" },
    ]});
    
    let result = slice.get_range_mut(0..3);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 3);
}

#[test]
fn test_get_range_mut_empty_range() {
    let mut slice = Box::new(Slice { entries: [] });
    
    let result = slice.get_range_mut(0..0);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 0);
}

#[test]
fn test_get_range_mut_part_range() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: Default::default(), key: 1, value: "one" },
        Bucket { hash: Default::default(), key: 2, value: "two" },
        Bucket { hash: Default::default(), key: 3, value: "three" },
    ]});
    
    let result = slice.get_range_mut(1..3);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 2);
}

#[test]
fn test_get_range_mut_excluded_end() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: Default::default(), key: 1, value: "one" },
        Bucket { hash: Default::default(), key: 2, value: "two" },
        Bucket { hash: Default::default(), key: 3, value: "three" },
    ]});
    
    let result = slice.get_range_mut(0..2);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 2);
}

#[test]
fn test_get_range_mut_out_of_bounds() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: Default::default(), key: 1, value: "one" },
        Bucket { hash: Default::default(), key: 2, value: "two" },
    ]});
    
    let result = slice.get_range_mut(2..4);
    assert!(result.is_none());
}

#[test]
fn test_get_range_mut_start_greater_than_end() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: Default::default(), key: 1, value: "one" },
        Bucket { hash: Default::default(), key: 2, value: "two" },
    ]});
    
    let result = slice.get_range_mut(1..0);
    assert!(result.is_none());
}

