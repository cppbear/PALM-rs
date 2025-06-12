// Answer 0

#[test]
fn test_zeroed_creates_bytesmut_with_correct_length_and_capacity() {
    let len = 42;
    let zeros = zeroed(len);
    
    assert!(zeros.capacity() >= len);
    assert_eq!(zeros.len(), len);
    zeros.into_iter().for_each(|x| assert_eq!(x, 0));
}

#[test]
fn test_zeroed_creates_empty_bytesmut() {
    let len = 0;
    let zeros = zeroed(len);
    
    assert!(zeros.capacity() >= len);
    assert_eq!(zeros.len(), len);
}

#[test]
fn test_zeroed_creates_bytesmut_with_large_length() {
    let len = 1024;
    let zeros = zeroed(len);
    
    assert!(zeros.capacity() >= len);
    assert_eq!(zeros.len(), len);
    zeros.into_iter().for_each(|x| assert_eq!(x, 0));
}

