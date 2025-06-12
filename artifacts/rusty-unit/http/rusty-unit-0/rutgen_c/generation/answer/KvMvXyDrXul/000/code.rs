// Answer 0

#[test]
fn test_extensions_mut_success() {
    struct TestMethod;
    impl TryInto<Method> for TestMethod {
        type Error = crate::Error;

        fn try_into(self) -> Result<Method> {
            Ok(Method::GET) // assuming GET is a valid Method
        }
    }

    struct TestUri;
    impl TryInto<Uri> for TestUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Ok(Uri::from_static("http://example.com")) // assuming valid Uri
        }
    }

    let mut builder = Builder::new()
        .method(TestMethod)
        .uri(TestUri)
        .extensions_mut().unwrap(); // Initialize extensions

    assert!(builder.extensions_mut().is_some());
}

#[test]
fn test_extensions_mut_error() {
    struct TestMethodThatFails;
    impl TryInto<Method> for TestMethodThatFails {
        type Error = crate::Error;

        fn try_into(self) -> Result<Method> {
            Err(crate::Error { inner: ErrorKind::Something }) // Assume an error occurs
        }
    }

    let mut builder = Builder::new()
        .method(TestMethodThatFails); // This should lead to an error

    assert!(builder.extensions_mut().is_none());
}

#[test]
fn test_extensions_mut_modify() {
    struct TestMethod;
    impl TryInto<Method> for TestMethod {
        type Error = crate::Error;

        fn try_into(self) -> Result<Method> {
            Ok(Method::GET) // assuming GET is a valid Method
        }
    }

    struct TestUri;
    impl TryInto<Uri> for TestUri {
        type Error = crate::Error;

        fn try_into(self) -> Result<Uri> {
            Ok(Uri::from_static("http://example.com")) // assuming valid Uri
        }
    }

    let mut builder = Builder::new()
        .method(TestMethod)
        .uri(TestUri)
        .extension("Initial Extension");

    let extensions = builder.extensions_mut().unwrap();
    assert_eq!(extensions.get::<&'static str>(), Some(&"Initial Extension"));

    extensions.insert(5u32);
    assert_eq!(extensions.get::<u32>(), Some(&5u32));
}

