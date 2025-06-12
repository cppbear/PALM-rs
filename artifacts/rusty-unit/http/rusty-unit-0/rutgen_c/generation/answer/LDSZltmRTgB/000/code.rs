// Answer 0

#[test]
fn test_delete_builder_with_valid_uri() {
    use crate::Uri;

    struct ValidUri(String);
    impl TryInto<Uri> for ValidUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Ok(Uri::from(self.0))
        }
    }

    let builder = Request::delete(ValidUri("https://www.rust-lang.org/".to_string()));
    assert!(builder.inner.is_ok());
}

#[test]
#[should_panic]
fn test_delete_builder_with_invalid_uri() {
    use crate::Uri;

    struct InvalidUri;

    impl TryInto<Uri> for InvalidUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Err(crate::Error::from("Invalid URI"))
        }
    }

    let _builder = Request::delete(InvalidUri);
}

#[test]
fn test_delete_builder_default_method() {
    let builder = Request::delete("https://www.rust-lang.org/");
    let inner = builder.inner.unwrap();
    assert_eq!(inner.method, Method::DELETE);
}

