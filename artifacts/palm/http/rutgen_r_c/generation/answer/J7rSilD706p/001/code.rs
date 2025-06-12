// Answer 0

#[test]
fn test_request_head_valid_uri() {
    use std::convert::TryInto;
    use crate::{Uri, Request};

    let valid_uri: &str = "https://www.rust-lang.org/";
    let builder = Request::head(valid_uri);
    let uri = valid_uri.try_into().unwrap(); // Ensure this conversion works
    assert_eq!(builder.method_ref().unwrap(), &Method::HEAD);
    assert_eq!(builder.uri_ref().unwrap(), &uri);
}

#[test]
#[should_panic]
fn test_request_head_invalid_uri() {
    use std::convert::TryInto;
    use crate::{Uri, Request};

    let invalid_uri: &str = "ht!tp://invalid.uri"; // Invalid URI format
    let _builder = Request::head(invalid_uri); // This should panic on conversion
}

#[test]
fn test_request_head_empty_uri() {
    use std::convert::TryInto;
    use crate::{Uri, Request};

    let empty_uri: &str = "";
    let builder = Request::head(empty_uri);
    let result = empty_uri.try_into();
    assert!(result.is_err()); // Ensure the conversion fails for an empty string
    assert_eq!(builder.uri_ref(), None);
}

