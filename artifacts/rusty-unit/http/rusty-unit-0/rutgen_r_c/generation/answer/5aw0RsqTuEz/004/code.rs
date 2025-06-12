// Answer 0

#[test]
fn test_try_reserve_success_when_empty() {
    // Initialize a HeaderMap with the desired capacity
    let mut header_map: http::HeaderMap = http::HeaderMap::with_capacity(0);
    header_map.entries.push(http::Bucket {
        hash: 0,
        key: http::HeaderName::from_static("example"),
        value: http::HeaderValue::from_static("value"),
        links: None,
    });

    // Attempt to reserve capacity for additional headers
    let result = header_map.try_reserve(1);

    // Assert that the reservation was successful
    assert_eq!(result, Ok(()));
}

#[test]
fn test_try_reserve_failure_due_to_max_size() {
    let mut header_map: http::HeaderMap = http::HeaderMap::with_capacity(1);
    header_map.entries.push(http::Bucket {
        hash: 0,
        key: http::HeaderName::from_static("example"),
        value: http::HeaderValue::from_static("value"),
        links: None,
    });

    // Fill entries to near maximum size
    for _ in 0..http::MAX_SIZE - 1 {
        header_map.entries.push(http::Bucket {
            hash: 0,
            key: http::HeaderName::from_static("example"),
            value: http::HeaderValue::from_static("value"),
            links: None,
        });
    }

    // Now try to reserve additional space which should fail.
    let result = header_map.try_reserve(1);
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_multiple() {
    // Start with an empty HeaderMap
    let mut header_map: http::HeaderMap = http::HeaderMap::with_capacity(0);

    // Try reserving a significant amount of space
    let result = header_map.try_reserve(128);

    // Check for success
    assert_eq!(result, Ok(()));
    assert!(header_map.entries.capacity() >= 128);
}

#[test]
fn test_try_reserve_no_overflow() {
    // Create a HeaderMap and fill it to just below the maximum size
    let mut header_map: http::HeaderMap = http::HeaderMap::with_capacity(0);

    // Fill to maximum capacity minus one
    for _ in 0..http::MAX_SIZE - 1 {
        header_map.entries.push(http::Bucket {
            hash: 0,
            key: http::HeaderName::from_static("example"),
            value: http::HeaderValue::from_static("value"),
            links: None,
        });
    }

    // Try reserving one more
    let result = header_map.try_reserve(1);
    assert!(result.is_err());
}

