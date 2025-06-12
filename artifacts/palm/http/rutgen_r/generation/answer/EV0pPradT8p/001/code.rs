// Answer 0

#[test]
fn test_extensions_mut_with_valid_response() {
    struct MockResponse {
        inner: Option<ResponseInner>,
    }

    struct ResponseInner {
        extensions: Extensions,
    }
    
    impl MockResponse {
        fn builder() -> Self {
            Self {
                inner: Some(ResponseInner {
                    extensions: Extensions::new(),
                }),
            }
        }

        fn extensions_mut(&mut self) -> Option<&mut Extensions> {
            self.inner.as_mut().map(|h| &mut h.extensions)
        }
    }
    
    let mut res = MockResponse::builder();
    let mut extensions = res.extensions_mut().unwrap();
    extensions.insert("My Extension");
    assert_eq!(extensions.get::<&str>(), Some(&"My Extension"));
    
    extensions.insert(5u32);
    assert_eq!(extensions.get::<u32>(), Some(&5u32));
}

#[test]
fn test_extensions_mut_with_error_response() {
    struct MockResponse {
        inner: Option<ResponseInner>,
    }

    struct ResponseInner {
        extensions: Extensions,
    }

    impl MockResponse {
        fn builder() -> Self {
            Self { inner: None }
        }

        fn extensions_mut(&mut self) -> Option<&mut Extensions> {
            self.inner.as_mut().map(|h| &mut h.extensions)
        }
    }

    let mut res = MockResponse::builder();
    let extensions = res.extensions_mut();
    assert!(extensions.is_none());
}

