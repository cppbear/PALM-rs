// Answer 0

#[test]
fn test_extensions_ref_with_extensions() {
    struct RequestBuilder {
        inner: Result<Inner, ()>,
    }

    struct Inner {
        extensions: Extensions,
    }

    struct Extensions {
        data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    }

    impl RequestBuilder {
        fn builder() -> Self {
            RequestBuilder {
                inner: Ok(Inner {
                    extensions: Extensions {
                        data: std::collections::HashMap::new(),
                    },
                }),
            }
        }

        fn extension<T: 'static>(&mut self, ext: T) -> &mut Self {
            if let Ok(ref mut inner) = self.inner {
                inner.extensions.data.insert(std::any::type_name::<T>().to_string(), Box::new(ext));
            }
            self
        }

        fn extensions_ref(&self) -> Option<&Extensions> {
            self.inner.as_ref().ok().map(|h| &h.extensions)
        }
    }

    let req = RequestBuilder::builder().extension("My Extension").extension(5u32);
    let extensions = req.extensions_ref().unwrap();
    assert_eq!(extensions.data.get(&std::any::type_name::<&'static str>().to_string()).and_then(|b| b.downcast_ref::<&'static str>()), Some(&"My Extension"));
    assert_eq!(extensions.data.get(&std::any::type_name::<u32>().to_string()).and_then(|b| b.downcast_ref::<u32>()), Some(&5u32));
}

#[test]
fn test_extensions_ref_with_error() {
    struct RequestBuilder {
        inner: Result<Inner, ()>,
    }

    struct Inner {
        extensions: Extensions,
    }

    struct Extensions {
        data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    }

    impl RequestBuilder {
        fn builder() -> Self {
            RequestBuilder {
                inner: Err(()),
            }
        }

        fn extensions_ref(&self) -> Option<&Extensions> {
            self.inner.as_ref().ok().map(|h| &h.extensions)
        }
    }

    let req = RequestBuilder::builder();
    let extensions = req.extensions_ref();
    assert_eq!(extensions, None);
}

