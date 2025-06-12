// Answer 0

#[test]
fn test_is_success_with_success_code_200() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(200) });
    assert!(status_code.is_success());
}

#[test]
fn test_is_success_with_success_code_299() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(299) });
    assert!(status_code.is_success());
}

#[test]
fn test_is_success_with_success_code_201() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(201) });
    assert!(status_code.is_success());
}

#[test]
fn test_is_success_with_non_success_code_199() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(199) });
    assert!(!status_code.is_success());
}

#[test]
fn test_is_success_with_non_success_code_300() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(300) });
    assert!(!status_code.is_success());
}

#[test]
fn test_is_success_with_non_success_code_404() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(404) });
    assert!(!status_code.is_success());
}

#[test]
fn test_is_success_with_non_success_code_500() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(500) });
    assert!(!status_code.is_success());
}

