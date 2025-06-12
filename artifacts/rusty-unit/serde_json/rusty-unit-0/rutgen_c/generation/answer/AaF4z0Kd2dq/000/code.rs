// Answer 0

#[test]
fn test_next_with_empty_slice() {
    let mut reader = SliceRead { slice: &[], index: 0 };
    let result = reader.next();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_next_with_non_empty_slice() {
    let data: &[u8] = &[1, 2, 3];
    let mut reader = SliceRead { slice: data, index: 0 };
    let result1 = reader.next();
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), Some(1));
    
    let result2 = reader.next();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), Some(2));
    
    let result3 = reader.next();
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap(), Some(3));
    
    let result4 = reader.next();
    assert!(result4.is_ok());
    assert_eq!(result4.unwrap(), None);
}

#[test]
fn test_next_with_boundary_conditions() {
    let data: &[u8] = &[10, 20, 30, 40];
    let mut reader = SliceRead { slice: data, index: 3 };
    
    let result = reader.next();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(40));
    
    let result_after_last = reader.next();
    assert!(result_after_last.is_ok());
    assert_eq!(result_after_last.unwrap(), None);
}

