// Answer 0

#[test]
fn test_extensions_mut_insert_and_get() {
    struct Request<T> {
        head: Head<T>,
    }
    
    struct Head<T> {
        extensions: Extensions,
        _marker: std::marker::PhantomData<T>,
    }

    struct Extensions {
        data: Option<String>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { data: None }
        }

        fn insert(&mut self, value: String) {
            self.data = Some(value);
        }

        fn get(&self) -> Option<&String> {
            self.data.as_ref()
        }
    }

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Request {
                head: Head {
                    extensions: Extensions::new(),
                    _marker: std::marker::PhantomData,
                },
            }
        }
    }

    impl<T> Request<T> {
        pub fn extensions_mut(&mut self) -> &mut Extensions {
            &mut self.head.extensions
        }

        pub fn extensions(&self) -> &Extensions {
            &self.head.extensions
        }
    }

    let mut request: Request<()> = Request::default();
    request.extensions_mut().insert("hello".to_string());
    assert_eq!(request.extensions().get(), Some(&"hello".to_string()));
}

#[test]
fn test_extensions_mut_empty() {
    struct Request<T> {
        head: Head<T>,
    }
    
    struct Head<T> {
        extensions: Extensions,
        _marker: std::marker::PhantomData<T>,
    }

    struct Extensions {
        data: Option<String>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions { data: None }
        }

        fn insert(&mut self, value: String) {
            self.data = Some(value);
        }

        fn get(&self) -> Option<&String> {
            self.data.as_ref()
        }
    }

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Request {
                head: Head {
                    extensions: Extensions::new(),
                    _marker: std::marker::PhantomData,
                },
            }
        }
    }

    impl<T> Request<T> {
        pub fn extensions_mut(&mut self) -> &mut Extensions {
            &mut self.head.extensions
        }

        pub fn extensions(&self) -> &Extensions {
            &self.head.extensions
        }
    }

    let request: Request<()> = Request::default();
    assert_eq!(request.extensions().get(), None);
}

