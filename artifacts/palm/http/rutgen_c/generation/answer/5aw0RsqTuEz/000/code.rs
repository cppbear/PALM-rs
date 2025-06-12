// Answer 0

#[test]
fn test_try_reserve_success() {
    let mut map = HeaderMap::with_capacity(5);
    assert!(map.try_reserve(10).is_ok());
    assert!(map.capacity() >= 15);
}

#[test]
fn test_try_reserve_empty() {
    let mut map = HeaderMap::with_capacity(0);
    assert!(map.try_reserve(5).is_ok());
    assert_eq!(map.capacity(), 8); // First power of two greater than 5
}

#[test]
fn test_try_reserve_exceeding_max_size() {
    let mut map = HeaderMap::with_capacity(0);
    let result = map.try_reserve(u16::MAX as usize);
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_overflow() {
    let mut map = HeaderMap::with_capacity(0);
    let result = map.try_reserve(usize::MAX);
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_when_full() {
    let mut map = HeaderMap::with_capacity(1);
    map.try_reserve(1).unwrap();
    assert!(map.try_reserve(1).is_ok());
    // Reserve another beyond capacity
    assert!(map.try_reserve(1).is_err());
}

