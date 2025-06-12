// Answer 0

#[test]
fn test_extensions_initially_none() {
    struct Response<T> {
        head: Head<T>,
    }

    struct Head<T> {
        extensions: Extensions,
    }

    struct Extensions {
        // Represents the internal storage of extensions
        data: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions {
                data: std::collections::HashMap::new(),
            }
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            self.data.get(&std::any::TypeId::of::<T>())
                .and_then(|b| b.downcast_ref::<T>())
        }
    }

    impl<T> Default for Response<T> {
        fn default() -> Self {
            Response {
                head: Head {
                    extensions: Extensions::new(),
                },
            }
        }
    }

    let response: Response<()> = Response::default();
    assert!(response.extensions().get::<i32>().is_none());
}

