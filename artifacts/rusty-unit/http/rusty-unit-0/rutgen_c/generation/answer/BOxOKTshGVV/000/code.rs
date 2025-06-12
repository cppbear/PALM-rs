// Answer 0

#[test]
fn test_options_with_valid_uri() {
    struct TestUri(String);
    
    impl TryInto<Uri> for TestUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Ok(Uri::from(self.0))
        }
    }

    let uri = TestUri("https://www.rust-lang.org/".to_string());
    let builder = Request::options(uri);
    let request = builder.body(()).unwrap();
    
    assert_eq!(*request.method(), Method::OPTIONS);
}

#[test]
#[should_panic]
fn test_options_with_invalid_uri() {
    struct InvalidUri;

    impl TryInto<Uri> for InvalidUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Err(crate::Error::from("invalid uri"))
        }
    }

    let uri = InvalidUri;
    let _builder = Request::options(uri);
}

#[test]
fn test_options_builder_has_correct_uri() {
    struct TestUri(String);
    
    impl TryInto<Uri> for TestUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Ok(Uri::from(self.0))
        }
    }

    let uri = TestUri("https://www.rust-lang.org/".to_string());
    let builder = Request::options(uri);
    let request = builder.body(()).unwrap();
    
    assert_eq!(request.uri_ref(), Some(&Uri::from("https://www.rust-lang.org/")));
}

