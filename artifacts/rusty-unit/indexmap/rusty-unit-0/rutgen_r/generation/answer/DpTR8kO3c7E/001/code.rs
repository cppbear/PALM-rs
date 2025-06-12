// Answer 0

#[test]
fn test_split_at_panics_when_index_greater_than_len() {
    let slice: Vec<i32> = vec![1, 2, 3];
    let index = 4;
    
    let result = std::panic::catch_unwind(|| {
        slice.split_at(index);
    });

    assert!(result.is_err());
}

#[test]
fn test_split_at_when_index_is_zero() {
    let slice: Vec<i32> = vec![1, 2, 3];
    let index = 0;
    
    let (first, second) = slice.split_at(index);
    
    assert_eq!(first, &[]);
    assert_eq!(second, &[1, 2, 3]);
}

#[test]
fn test_split_at_when_index_equals_len() {
    let slice: Vec<i32> = vec![1, 2, 3];
    let index = 3;
    
    let (first, second) = slice.split_at(index);
    
    assert_eq!(first, &[1, 2, 3]);
    assert_eq!(second, &[]);
}

#[test]
fn test_split_at_when_index_is_one() {
    let slice: Vec<i32> = vec![1, 2, 3];
    let index = 1;
    
    let (first, second) = slice.split_at(index);
    
    assert_eq!(first, &[1]);
    assert_eq!(second, &[2, 3]);
}

#[test]
fn test_split_at_when_index_is_two() {
    let slice: Vec<i32> = vec![1, 2, 3];
    let index = 2;
    
    let (first, second) = slice.split_at(index);
    
    assert_eq!(first, &[1, 2]);
    assert_eq!(second, &[3]);
}

