// Answer 0

#[test]
fn test_get_range_mut_valid_range() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" }
    ]});
    
    let result = slice.get_range_mut(1..3);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 2);
}

#[test]
fn test_get_range_mut_empty_slice() {
    let mut slice = Box::new(Slice { entries: [] });
    
    let result = slice.get_range_mut(0..1);
    assert!(result.is_none());
}

#[test]
fn test_get_range_mut_out_of_bounds() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: "a" }
    ]});
    
    let result = slice.get_range_mut(1..3);
    assert!(result.is_none());
}

#[test]
fn test_get_range_mut_full_range() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" }
    ]});
    
    let result = slice.get_range_mut(..);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 2);
}

#[test]
fn test_get_range_mut_inclusive_range() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" }
    ]});
    
    let result = slice.get_range_mut(0..=2);
    assert!(result.is_some());
    let result_slice = result.unwrap();
    assert_eq!(result_slice.len(), 3);
}

