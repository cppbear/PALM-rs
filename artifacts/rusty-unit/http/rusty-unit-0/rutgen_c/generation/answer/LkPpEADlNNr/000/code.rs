// Answer 0

#[test]
fn test_uri_with_valid_string() {
    use crate::{Request, Method};
    use std::convert::TryInto;

    struct MockMethod;
    impl TryInto<Method> for MockMethod {
        type Error = crate::Error;

        fn try_into(self) -> Result<Method, Self::Error> {
            Ok(Method::GET) // Replace with actual valid method
        }
    }

    struct MockUri;
    impl TryInto<Uri> for MockUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri, Self::Error> {
            Ok(Uri::from_static("https://www.rust-lang.org/")) // Use valid Uri
        }
    }

    let req = Request::builder()
        .method(MockMethod)
        .uri(MockUri)
        .body(())
        .unwrap();

    assert_eq!(req.uri().to_string(), "https://www.rust-lang.org/");
}

#[test]
#[should_panic]
fn test_uri_with_invalid_uri() {
    use crate::{Request, Method};
    use std::convert::TryInto;

    struct MockInvalidUri;
    impl TryInto<Uri> for MockInvalidUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri, Self::Error> {
            Err(crate::Error::new("Invalid URI")) // Simulate invalid URI error
        }
    }

    let req = Request::builder()
        .method(Method::GET) // Use valid method
        .uri(MockInvalidUri)
        .body(())
        .unwrap();
}

#[test]
fn test_uri_with_default() {
    use crate::{Request, Method};

    let req = Request::builder()
        .method(Method::GET) // Use valid method
        .body(())
        .unwrap();

    assert_eq!(req.uri().to_string(), "/");
}

