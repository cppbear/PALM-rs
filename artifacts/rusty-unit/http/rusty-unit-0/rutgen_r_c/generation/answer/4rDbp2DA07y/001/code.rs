// Answer 0

#[test]
fn test_as_u16_valid_status_code() {
    struct TestStatusCode(NonZeroU16);
    
    let status = TestStatusCode(NonZeroU16::new(200).unwrap());
    assert_eq!(status.as_u16(), 200);
    
    let status = TestStatusCode(NonZeroU16::new(404).unwrap());
    assert_eq!(status.as_u16(), 404);
    
    let status = TestStatusCode(NonZeroU16::new(500).unwrap());
    assert_eq!(status.as_u16(), 500);
}

#[test]
#[should_panic]
fn test_as_u16_zero_status_code() {
    struct TestStatusCode(NonZeroU16);
    
    // This is not a valid NonZeroU16, should panic.
    let _ = TestStatusCode(NonZeroU16::new(0).unwrap());
}

#[test]
fn test_as_u16_min_valid() {
    struct TestStatusCode(NonZeroU16);
    
    let status = TestStatusCode(NonZeroU16::new(1).unwrap());
    assert_eq!(status.as_u16(), 1);
}

#[test]
fn test_as_u16_max_valid() {
    struct TestStatusCode(NonZeroU16);
    
    let status = TestStatusCode(NonZeroU16::new(65535).unwrap());
    assert_eq!(status.as_u16(), 65535);
}

