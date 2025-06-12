// Answer 0

#[test]
fn test_response_debug_fmt() {
    use std::fmt::Formatter;
    use std::num::NonZeroU16;

    struct MockBody;

    let status_code = StatusCode(NonZeroU16::new(200).unwrap());
    let version = Version(Http);
    let headers = HeaderMap::default();
    let parts = Parts {
        status: status_code,
        version: version,
        headers: headers,
        extensions: Extensions::default(),
        _priv: (),
    };

    let response = Response::from_parts(parts, MockBody);

    let mut formatter = Formatter::new();
    let result = response.fmt(&mut formatter);
    
    assert!(result.is_ok());
}

