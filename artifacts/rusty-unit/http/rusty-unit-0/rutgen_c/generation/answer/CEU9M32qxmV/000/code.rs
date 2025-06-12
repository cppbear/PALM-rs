// Answer 0

#[test]
fn test_request_put_with_valid_uri() {
    struct ValidUri;
    impl TryInto<Uri> for ValidUri {
        type Error = crate::Error;
        fn try_into(self) -> Result<Uri, Self::Error> {
            Ok(Uri::from_static("https://www.rust-lang.org/"))
        }
    }

    let builder = Request::put(ValidUri);
    assert!(builder.method_ref().is_some());
    assert_eq!(*builder.method_ref().unwrap(), Method(MethodInner::PUT));
}

#[test]
fn test_request_put_with_invalid_uri() {
    struct InvalidUri;
    impl TryInto<Uri> for InvalidUri {
        type Error = crate::Error;
        fn try_into(self) -> Result<Uri, Self::Error> {
            Err(crate::Error::from("Invalid URI"))
        }
    }

    let builder = Request::put(InvalidUri);
    assert!(builder.method_ref().is_some());
    assert_eq!(*builder.method_ref().unwrap(), Method(MethodInner::PUT));
}

