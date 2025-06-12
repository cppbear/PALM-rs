// Answer 0

#[test]
fn test_with_capacity_zero() {
    struct TestValue;

    let map: HeaderMap<TestValue> = HeaderMap::with_capacity(0);

    assert!(map.is_empty());
    assert_eq!(0, map.capacity());
}

#[test]
fn test_with_capacity_below_max_size() {
    struct TestValue;

    let capacity = 10;
    let map: HeaderMap<TestValue> = HeaderMap::with_capacity(capacity);

    assert!(map.is_empty());
    assert!(map.capacity() >= capacity);
}

#[test]
#[should_panic]
fn test_with_capacity_too_large() {
    struct TestValue;

    let capacity = (1 << 16) + 1; // One more than MAX_SIZE
    let _map: HeaderMap<TestValue> = HeaderMap::with_capacity(capacity);
}

#[test]
fn test_with_capacity_large_enough() {
    struct TestValue;

    let capacity = 256;
    let map: HeaderMap<TestValue> = HeaderMap::with_capacity(capacity);

    assert!(map.is_empty());
    assert!(map.capacity() >= capacity);
    assert!(map.capacity() <= 512); // assuming that the underlying logic holds to power of two allocation
}

