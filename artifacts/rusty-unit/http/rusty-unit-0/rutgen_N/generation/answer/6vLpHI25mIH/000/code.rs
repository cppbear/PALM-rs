// Answer 0

#[test]
fn test_extensions_mut_insertion() {
    struct Extensions {
        data: Vec<String>,
    }

    impl Extensions {
        fn insert(&mut self, value: &str) {
            self.data.push(value.to_string());
        }

        fn get(&self) -> Option<&String> {
            self.data.last()
        }
    }

    struct Head {
        extensions: Extensions,
    }

    struct Response<T> {
        head: Head,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Default for Response<T> {
        fn default() -> Self {
            Response {
                head: Head {
                    extensions: Extensions { data: Vec::new() },
                },
                _marker: std::marker::PhantomData,
            }
        }
    }

    let mut response: Response<()> = Response::default();
    response.extensions_mut().insert("hello");
    assert_eq!(response.extensions_mut().get(), Some(&"hello".to_string()));
}

#[test]
fn test_extensions_mut_initial_state() {
    struct Extensions {
        data: Vec<String>,
    }

    impl Extensions {
        fn get(&self) -> Option<&String> {
            self.data.last()
        }
    }

    struct Head {
        extensions: Extensions,
    }

    struct Response<T> {
        head: Head,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Default for Response<T> {
        fn default() -> Self {
            Response {
                head: Head {
                    extensions: Extensions { data: Vec::new() },
                },
                _marker: std::marker::PhantomData,
            }
        }
    }

    let response: Response<()> = Response::default();
    assert_eq!(response.head.extensions.get(), None);
}

