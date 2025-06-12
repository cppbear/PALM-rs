// Answer 0

#[test]
fn test_extensions_mut_with_no_error() {
    struct Response {
        inner: Result<Inner, ()>,
    }

    struct Inner {
        extensions: Extensions,
    }

    struct Extensions {
        data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions {
                data: std::collections::HashMap::new(),
            }
        }

        fn get<T: std::any::Any>(&self) -> Option<&T> {
            self.data.get(&std::any::TypeId::of::<T>().to_string())
                .and_then(|v| v.downcast_ref::<T>())
        }

        fn insert<T: 'static>(&mut self, value: T) {
            self.data.insert(std::any::TypeId::of::<T>().to_string(), Box::new(value));
        }
    }

    impl Response {
        fn builder() -> Self {
            Response {
                inner: Ok(Inner { extensions: Extensions::new() }),
            }
        }

        fn extensions_mut(&mut self) -> Option<&mut Extensions> {
            self.inner.as_mut().ok().map(|h| &mut h.extensions)
        }
    }

    let mut res = Response::builder();
    let extensions = res.extensions_mut().unwrap();
    assert_eq!(extensions.get::<&'static str>(), None);
    extensions.insert("My Extension");
    assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
    extensions.insert(5u32);
    assert_eq!(extensions.get::<u32>(), Some(&5u32));
}

#[test]
fn test_extensions_mut_with_error() {
    struct Response {
        inner: Result<Inner, ()>,
    }

    struct Inner {
        extensions: Extensions,
    }

    struct Extensions {
        data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions {
                data: std::collections::HashMap::new(),
            }
        }
    }

    impl Response {
        fn builder() -> Self {
            Response {
                inner: Err(()),
            }
        }
        
        fn extensions_mut(&mut self) -> Option<&mut Extensions> {
            self.inner.as_mut().ok().map(|h| &mut h.extensions)
        }
    }

    let mut res = Response::builder();
    assert_eq!(res.extensions_mut(), None);
}

