// Answer 0

#[test]
fn test_is_empty_with_no_elements() {
    let slice = Slice::<i32, i32>::new();
    slice.is_empty();
}

#[test]
fn test_is_empty_with_single_element() {
    let mut entries = vec![Bucket { hash: 0, key: 1, value: 10 }];
    let slice = Slice { entries: entries.try_into().unwrap() };
    slice.is_empty();
}

#[test]
fn test_is_empty_with_one_element() {
    let mut entries = vec![Bucket { hash: 0, key: 1, value: 10 }];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let result = slice.len();
    assert_eq!(result, 1);
    slice.is_empty();
}

#[test]
fn test_is_empty_with_two_elements() {
    let mut entries = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let result = slice.len();
    assert_eq!(result, 2);
    slice.is_empty();
}

#[test]
fn test_is_empty_with_max_elements() {
    let mut entries: Vec<Bucket<u32, u32>> = (0..usize::MAX)
        .map(|i| Bucket { hash: i as u64, key: i, value: i as u32 })
        .collect();
    let slice = Slice { entries: entries.try_into().unwrap() };
    let result = slice.len();
    assert!(result > 1);
    slice.is_empty();
}

#[test]
fn test_is_empty_with_half_elements() {
    let mut entries: Vec<Bucket<u32, u32>> = (0..usize::MAX / 2)
        .map(|i| Bucket { hash: i as u64, key: i, value: i as u32 })
        .collect();
    let slice = Slice { entries: entries.try_into().unwrap() };
    let result = slice.len();
    assert!(result > 0);
    slice.is_empty();
}

#[test]
fn test_is_empty_with_edge_case_zero_to_zero() {
    let slice = Slice::<u32, u32>::new();
    assert!(slice.is_empty());
}

#[test]
fn test_is_empty_with_edge_case_one_to_one() {
    let entries = vec![Bucket { hash: 0, key: 1, value: 10 }];
    let slice = Slice { entries: entries.try_into().unwrap() };
    assert!(!slice.is_empty());
}

#[test]
fn test_is_empty_with_edge_case_zero_to_max() {
    let entries: Vec<Bucket<u32, u32>> = (0..usize::MAX).map(|i| Bucket { hash: i as u64, key: i, value: i as u32 }).collect();
    let slice = Slice { entries: entries.try_into().unwrap() };
    assert!(!slice.is_empty());
}

#[test]
fn test_is_empty_with_edge_case_max_to_max() {
    let entries: Vec<Bucket<u32, u32>> = vec![Bucket { hash: 0, key: usize::MAX, value: 10 }];
    let slice = Slice { entries: entries.try_into().unwrap() };
    assert!(!slice.is_empty());
}

