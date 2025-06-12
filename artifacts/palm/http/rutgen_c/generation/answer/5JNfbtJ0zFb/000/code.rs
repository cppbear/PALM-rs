// Answer 0

#[test]
fn test_extensions_ref_valid() {
    use std::any::Any;

    struct DummyMethod;

    impl TryInto<Method> for DummyMethod {
        type Error = crate::Error;
        fn try_into(self) -> Result<Method, Self::Error> {
            Ok(Method::GET) // Assuming GET is a valid method
        }
    }

    struct DummyUri;

    impl TryInto<Uri> for DummyUri {
        type Error = crate::Error;
        fn try_into(self) -> Result<Uri, Self::Error> {
            Ok(Uri::from_static("http://example.com")) // Example URI
        }
    }

    let extensions = Extensions::default().extension("My Extension").extension(5u32);
    let builder = Builder {
        inner: Ok(Parts {
            method: DummyMethod.try_into().unwrap(),
            uri: DummyUri.try_into().unwrap(),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            extensions,
            _priv: (),
        }),
    };

    let ext_ref = builder.extensions_ref().unwrap();
    assert_eq!(ext_ref.get::<&'static str>(), Some(&"My Extension"));
    assert_eq!(ext_ref.get::<u32>(), Some(&5u32));
}

#[test]
fn test_extensions_ref_error() {
    let builder = Builder {
        inner: Err(crate::Error::new("Some error")), // Simulating an error state
    };

    assert!(builder.extensions_ref().is_none());
}

