// Answer 0

#[test]
fn test_patch_with_valid_uri() {
    use crate::Uri;

    let valid_uri = "https://www.rust-lang.org/";
    let builder = Request::patch(valid_uri).uri(Uri::from_str(valid_uri).unwrap());
    assert!(builder.method_ref().is_some());
    assert_eq!(builder.method_ref().unwrap(), &Method::PATCH);
    assert_eq!(builder.uri_ref().unwrap().to_string(), valid_uri);
}

#[test]
#[should_panic]
fn test_patch_with_invalid_uri() {
    use std::str::FromStr;

    let invalid_uri = "htp://invalid_uri";
    let _ = Request::patch(invalid_uri).uri(Uri::from_str(invalid_uri).unwrap());
}

#[test]
fn test_patch_with_empty_uri() {
    use crate::Uri;

    let empty_uri = "";
    let builder = Request::patch(empty_uri).uri(Uri::from_str(empty_uri).unwrap_err());
    assert!(builder.method_ref().is_some());
    assert_eq!(builder.method_ref().unwrap(), &Method::PATCH);
    assert!(builder.uri_ref().is_none());
}

