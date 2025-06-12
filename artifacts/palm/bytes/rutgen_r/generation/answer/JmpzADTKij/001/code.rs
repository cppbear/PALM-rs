// Answer 0

#[test]
fn test_from_vec_empty() {
    let vec: Vec<u8> = Vec::new();
    let result = from_vec(vec);
    assert_eq!(result.len, 0);
    assert_eq!(result.cap, 0);
}

#[test]
fn test_from_vec_single_element() {
    let vec = vec![42];
    let result = from_vec(vec);
    assert_eq!(result.len, 1);
    assert_eq!(result.cap, 1);
}

#[test]
fn test_from_vec_multiple_elements() {
    let vec = vec![1, 2, 3, 4, 5];
    let result = from_vec(vec);
    assert_eq!(result.len, 5);
    assert_eq!(result.cap, 5);
}

#[test]
fn test_from_vec_large_capacity() {
    let vec = vec![0u8; 1024]; // vector with a large capacity
    let result = from_vec(vec);
    assert_eq!(result.len, 1024);
    assert!(result.cap >= 1024); // capacity should be at least 1024
}

#[test]
fn test_from_vec_capacity_greater_than_length() {
    let vec = vec![10, 20, 30]; // length is 3
    let result = from_vec(vec);
    assert_eq!(result.len, 3);
    assert!(result.cap > result.len); // Ensure cap is greater than len
}

