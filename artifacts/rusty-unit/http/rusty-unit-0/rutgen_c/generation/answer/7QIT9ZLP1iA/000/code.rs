// Answer 0

#[test]
fn test_parts_debug_fmt() {
    use std::num::NonZeroU16;
    use std::fmt::Debug;
    use crate::header::HeaderValue;

    let status_code = StatusCode(NonZeroU16::new(200).unwrap());
    let version = Version(0); // Assuming 0 represents HTTP/0.0
    let headers = HeaderMap::default(); // Assuming HeaderMap has a default implementation
    let extensions = Extensions::default();
    
    let parts = Parts {
        status: status_code,
        version: version,
        headers,
        extensions,
        _priv: (),
    };

    let mut output = String::new();
    let result = parts.fmt(&mut fmt::Formatter::new(output.as_mut_str()));

    assert!(result.is_ok());
}

#[test]
fn test_parts_debug_fmt_empty() {
    use std::num::NonZeroU16;
    use std::fmt::Debug;
    use crate::header::HeaderValue;

    let status_code = StatusCode(NonZeroU16::new(404).unwrap()); // Not Found
    let version = Version(0); // Assuming 0 represents HTTP/0.0
    let headers = HeaderMap::default(); // Assuming HeaderMap has a default implementation
    let extensions = Extensions::default();
    
    let parts = Parts {
        status: status_code,
        version: version,
        headers,
        extensions,
        _priv: (),
    };

    let mut output = String::new();
    let result = parts.fmt(&mut fmt::Formatter::new(output.as_mut_str()));

    assert!(result.is_ok());
}

