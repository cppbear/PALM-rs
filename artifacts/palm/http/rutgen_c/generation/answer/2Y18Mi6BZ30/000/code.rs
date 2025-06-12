// Answer 0

#[test]
fn test_post_uri_string() {
    struct Uri(String);
    impl TryInto<Uri> for &str {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Ok(Uri(self.to_string()))
        }
    }

    let builder = Request::post("https://www.rust-lang.org/");
    // You would need a way to access the inner representation of the Builder. Assuming it has a method to fetch the method.
    assert_eq!(builder.method_ref(), Some(Method::POST));
}

#[test]
fn test_post_valid_uri() {
    struct Uri(String);
    impl TryInto<Uri> for &str {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Ok(Uri(self.to_string()))
        }
    }

    let uri = "https://www.rust-lang.org/";
    let builder = Request::post(uri);
    assert!(builder.uri_ref().is_some());
}

#[test]
#[should_panic]
fn test_post_invalid_uri() {
    struct Uri(String);
    impl TryInto<Uri> for &str {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Err(crate::Error::from("Invalid URI"))
        }
    }

    let uri = "invalid_uri";
    let _builder = Request::post(uri); // Should panic due to invalid URI handling
}

