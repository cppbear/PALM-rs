// Answer 0

#[test]
fn test_from_lowercase_valid_standard_header1() {
    let hdr = HeaderName::from_lowercase(b"accept").unwrap();
}

#[test]
fn test_from_lowercase_valid_standard_header2() {
    let hdr = HeaderName::from_lowercase(b"content-type").unwrap();
}

#[test]
fn test_from_lowercase_valid_standard_header3() {
    let hdr = HeaderName::from_lowercase(b"accept-language").unwrap();
}

#[test]
fn test_from_lowercase_valid_standard_header4() {
    let hdr = HeaderName::from_lowercase(b"connection").unwrap();
}

#[test]
fn test_from_lowercase_valid_standard_header5() {
    let hdr = HeaderName::from_lowercase(b"user-agent").unwrap();
}

