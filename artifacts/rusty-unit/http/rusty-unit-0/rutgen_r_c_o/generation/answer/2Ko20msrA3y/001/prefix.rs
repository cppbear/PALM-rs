// Answer 0

#[test]
fn test_as_str_return_200() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(200) });
    let result = status.as_str();
}

#[test]
fn test_as_str_return_100() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(100) });
    let result = status.as_str();
}

#[test]
fn test_as_str_return_999() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(999) });
    let result = status.as_str();
}

#[test]
fn test_as_str_return_404() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(404) });
    let result = status.as_str();
}

#[test]
fn test_as_str_return_500() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(500) });
    let result = status.as_str();
}

#[test]
fn test_as_str_return_418() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(418) });
    let result = status.as_str();
}

