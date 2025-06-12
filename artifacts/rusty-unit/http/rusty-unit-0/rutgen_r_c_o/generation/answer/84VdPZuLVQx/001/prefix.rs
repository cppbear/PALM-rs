// Answer 0

#[test]
fn test_is_redirection_300() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(300) });
    let _ = status_code.is_redirection();
}

#[test]
fn test_is_redirection_301() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(301) });
    let _ = status_code.is_redirection();
}

#[test]
fn test_is_redirection_398() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(398) });
    let _ = status_code.is_redirection();
}

#[test]
fn test_is_redirection_399() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(399) });
    let _ = status_code.is_redirection();
}

#[test]
fn test_is_redirection_200() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(200) });
    let _ = status_code.is_redirection();
}

#[test]
fn test_is_redirection_404() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(404) });
    let _ = status_code.is_redirection();
}

