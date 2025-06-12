// Answer 0

#[test]
fn test_zeroed_with_zero_length() {
    let zeros = zeroed(0);
    assert!(zeros.capacity() >= 0);
    assert_eq!(zeros.len(), 0);
}

#[test]
fn test_zeroed_with_small_length() {
    let zeros = zeroed(1);
    assert!(zeros.capacity() >= 1);
    assert_eq!(zeros.len(), 1);
    assert_eq!(zeros.into_iter().next(), Some(0));
}

#[test]
fn test_zeroed_with_large_length() {
    let len = 10_000;
    let zeros = zeroed(len);
    assert!(zeros.capacity() >= len);
    assert_eq!(zeros.len(), len);
    zeros.into_iter().for_each(|x| assert_eq!(x, 0));
}

#[test]
fn test_zeroed_with_edge_case() {
    let zeros = zeroed(usize::MAX);
    assert!(zeros.capacity() >= usize::MAX);
    assert_eq!(zeros.len(), usize::MAX);
    zeros.into_iter().for_each(|x| assert_eq!(x, 0));
} 

#[should_panic]
fn test_zeroed_with_invalid_length() {
    let _zeros = zeroed(usize::MAX + 1);
}

