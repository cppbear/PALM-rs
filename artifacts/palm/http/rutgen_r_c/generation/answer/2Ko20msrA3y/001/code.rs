// Answer 0

#[test]
fn test_as_str_ok() {
    struct StatusCodeWrapper(StatusCode);

    let status = StatusCodeWrapper(StatusCode(unsafe { NonZeroU16::new_unchecked(200) }));
    assert_eq!(status.0.as_str(), "200");
}

#[test]
fn test_as_str_100() {
    struct StatusCodeWrapper(StatusCode);

    let status = StatusCodeWrapper(StatusCode(unsafe { NonZeroU16::new_unchecked(100) }));
    assert_eq!(status.0.as_str(), "100");
}

#[test]
fn test_as_str_999() {
    struct StatusCodeWrapper(StatusCode);
    
    let status = StatusCodeWrapper(StatusCode(unsafe { NonZeroU16::new_unchecked(999) }));
    assert_eq!(status.0.as_str(), "999");
}

#[should_panic]
#[test]
fn test_as_str_below_min() {
    struct StatusCodeWrapper(StatusCode);

    let status = StatusCodeWrapper(StatusCode(unsafe { NonZeroU16::new_unchecked(99) }));
    // This should panic because 99 is not a valid status code
    let _ = status.0.as_str();
}

#[should_panic]
#[test]
fn test_as_str_above_max() {
    struct StatusCodeWrapper(StatusCode);

    let status = StatusCodeWrapper(StatusCode(unsafe { NonZeroU16::new_unchecked(1000) }));
    // This should panic because 1000 is not a valid status code
    let _ = status.0.as_str();
}

