// Answer 0

#[test]
fn test_into_bytes_empty() {
    let header_name = HeaderName::from_bytes(b"").unwrap();
    header_name.into_bytes();
}

#[test]
fn test_into_bytes_single_character() {
    let header_name = HeaderName::from_bytes(b"a").unwrap();
    header_name.into_bytes();
}

#[test]
fn test_into_bytes_uppercase() {
    let header_name = HeaderName::from_bytes(b"HEADER").unwrap();
    header_name.into_bytes();
}

#[test]
fn test_into_bytes_lowercase() {
    let header_name = HeaderName::from_bytes(b"header").unwrap();
    header_name.into_bytes();
}

#[test]
fn test_into_bytes_special_characters() {
    let header_name = HeaderName::from_bytes(b"Header-1!").unwrap();
    header_name.into_bytes();
}

#[test]
fn test_into_bytes_max_length() {
    let header_name = HeaderName::from_bytes(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$%&'()*+,-./:;<=>?@[\\]^_`{|}~").unwrap();
    header_name.into_bytes();
}

#[test]
fn test_into_bytes_valid_range() {
    let header_name = HeaderName::from_bytes(b"Accept-Encoding").unwrap();
    header_name.into_bytes();
}

#[test]
fn test_into_bytes_all_valid_characters() {
    let header_name = HeaderName::from_bytes(b"!#$%&'*+-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz").unwrap();
    header_name.into_bytes();
}

