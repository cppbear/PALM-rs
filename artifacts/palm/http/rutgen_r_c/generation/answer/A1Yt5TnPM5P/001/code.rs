// Answer 0

#[test]
fn test_try_with_capacity_zero() {
    let result: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(0);
    assert!(result.is_ok());

    let map = result.unwrap();
    assert!(map.is_empty());
    assert_eq!(0, map.capacity());
}

#[test]
fn test_try_with_capacity_basic() {
    let result: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(10);
    assert!(result.is_ok());

    let map = result.unwrap();
    assert!(map.is_empty());
    assert!(map.capacity() >= 10); // ensures it's greater than or equal to requested
}

#[test]
fn test_try_with_capacity_maximum() {
    let result: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(8192);
    assert!(result.is_ok());

    let map = result.unwrap();
    assert!(map.capacity() >= 8192); // checks capacity is as expected
}

#[test]
#[should_panic]
fn test_try_with_capacity_overflow() {
    let _result: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(usize::MAX);
}

#[test]
fn test_try_with_capacity_multiply_three() {
    let result: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(3);
    assert!(result.is_ok());

    let map = result.unwrap();
    assert!(map.capacity() >= 3); // checks capacity is as expected
}

#[test]
fn test_try_with_capacity_boundary() {
    let result: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(128 * 4); // pushing boundary
    assert!(result.is_ok());

    let map = result.unwrap();
    assert!(map.capacity() >= 128); // checks capacity is as expected
}

