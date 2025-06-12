// Answer 0

#[test]
fn test_request_delete_valid_uri() {
    use crate::Uri;
    
    let uri: Uri = "https://www.rust-lang.org/".try_into().unwrap();
    let builder = Request::delete(uri);
    
    assert!(builder.method_ref().is_some());
    assert!(builder.method_ref().unwrap() == &Method::DELETE);
    assert!(builder.uri_ref().is_some());
}

#[test]
#[should_panic]
fn test_request_delete_invalid_uri() {
    use std::convert::Infallible;

    struct InvalidUri;
    impl TryInto<Uri> for InvalidUri {
        type Error = Infallible;
        
        fn try_into(self) -> Result<Uri, Self::Error> {
            Err(Infallible)
        }
    }

    let builder = Request::delete(InvalidUri);
}

#[test]
fn test_request_delete_empty_string_uri() {
    use crate::Uri;

    struct EmptyString;
    impl TryInto<Uri> for EmptyString {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri, Self::Error> {
            // Assume some validation logic may return Error for empty strings
            Err(crate::Error::from("Invalid URI"))
        }
    }

    let result = Request::delete(EmptyString);
    assert!(result.method_ref().is_some());
    assert!(result.method_ref().unwrap() == &Method::DELETE);
}

