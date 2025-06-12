// Answer 0

#[test]
fn test_connect_valid_uri() {
    struct ValidUri;
    impl TryInto<Uri> for ValidUri {
        type Error = crate::Error;
        fn try_into(self) -> Result<Uri> {
            Ok(Uri::new("https://www.rust-lang.org/"))
        }
    }

    let builder = Request::<()>::connect(ValidUri);
    let method = builder.method_ref().unwrap();
    let uri = builder.uri_ref().unwrap();

    assert_eq!(*method, Method::CONNECT);
    assert_eq!(uri.to_string(), "https://www.rust-lang.org/");
}

#[test]
#[should_panic]
fn test_connect_invalid_uri() {
    struct InvalidUri;
    impl TryInto<Uri> for InvalidUri {
        type Error = crate::Error;
        fn try_into(self) -> Result<Uri> {
            Err(crate::Error::new("Invalid URI"))
        }
    }

    let _ = Request::<()>::connect(InvalidUri);
}

#[test]
fn test_connect_empty_uri() {
    struct EmptyUri;
    impl TryInto<Uri> for EmptyUri {
        type Error = crate::Error;
        fn try_into(self) -> Result<Uri> {
            Ok(Uri::new(""))
        }
    }

    let builder = Request::<()>::connect(EmptyUri);
    let method = builder.method_ref().unwrap();
    let uri = builder.uri_ref().unwrap();

    assert_eq!(*method, Method::CONNECT);
    assert_eq!(uri.to_string(), "");
}

