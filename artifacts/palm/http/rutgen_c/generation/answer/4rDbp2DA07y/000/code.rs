// Answer 0

#[test]
fn test_as_u16_OK() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(200) });
    assert_eq!(status.as_u16(), 200);
}

#[test]
fn test_as_u16_CONTINUE() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(100) });
    assert_eq!(status.as_u16(), 100);
}

#[test]
fn test_as_u16_NOT_FOUND() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(404) });
    assert_eq!(status.as_u16(), 404);
}

#[test]
fn test_as_u16_INTERNAL_SERVER_ERROR() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(500) });
    assert_eq!(status.as_u16(), 500);
}

