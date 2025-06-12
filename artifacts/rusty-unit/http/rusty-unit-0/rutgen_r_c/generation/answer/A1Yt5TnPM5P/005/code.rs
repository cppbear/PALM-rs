// Answer 0

#[test]
fn test_try_with_capacity_zero() {
    let result = HeaderMap::<u32>::try_with_capacity(0);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_try_with_capacity_too_large() {
    let result = HeaderMap::<u32>::try_with_capacity(usize::MAX);
    assert!(result.is_err());
    assert!(result.err().unwrap()._priv.is_empty());
}

