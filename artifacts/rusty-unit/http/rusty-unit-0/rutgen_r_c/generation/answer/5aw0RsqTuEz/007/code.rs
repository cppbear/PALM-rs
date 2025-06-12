// Answer 0

#[test]
fn test_try_reserve_ok() {
    struct TestHeaderValue;

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(1);
    map.entries.push(Bucket {
        hash: HashValue(0),
        key: HeaderName::from_static("Test-Header"),
        value: TestHeaderValue,
        links: None,
    });
    assert!(map.try_reserve(1).is_ok());
}

#[test]
fn test_try_reserve_exceeding_capacity() {
    struct TestHeaderValue;

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(1);
    map.entries.push(Bucket {
        hash: HashValue(0),
        key: HeaderName::from_static("Test-Header"),
        value: TestHeaderValue,
        links: None,
    });
    map.entries.push(Bucket {
        hash: HashValue(1),
        key: HeaderName::from_static("Another-Header"),
        value: TestHeaderValue,
        links: None,
    });
    let result = map.try_reserve(u16::MAX as usize - 1);
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_no_growth() {
    struct TestHeaderValue;

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(4);
    assert!(map.try_reserve(2).is_ok());
    assert_eq!(map.capacity(), 4);  // Ensure capacity hasn't changed
}

#[test]
fn test_try_reserve_exceeding_max_size() {
    struct TestHeaderValue;

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(1);
    map.entries.push(Bucket {
        hash: HashValue(0),
        key: HeaderName::from_static("Test-Header"),
        value: TestHeaderValue,
        links: None,
    });
    
    let result = map.try_reserve(MAX_SIZE as usize);
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_exact_capacity() {
    struct TestHeaderValue;

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(1);
    map.entries.push(Bucket {
        hash: HashValue(0),
        key: HeaderName::from_static("Test-Header"),
        value: TestHeaderValue,
        links: None,
    });
    
    let cap_before = map.capacity();
    assert!(map.try_reserve(cap_before).is_ok());
    assert_eq!(map.capacity(), cap_before);
}

