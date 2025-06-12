// Answer 0

#[test]
fn test_extensions_ref_with_valid_extensions() {
    use http::Response;
    use std::collections::HashMap;

    struct Extensions {
        data: HashMap<&'static str, Box<dyn std::any::Any>>,
    }

    impl Extensions {
        fn new() -> Self {
            Extensions {
                data: HashMap::new(),
            }
        }

        fn insert<T: 'static>(&mut self, value: T) {
            self.data.insert(std::any::type_name::<T>(), Box::new(value));
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            self.data.get(std::any::type_name::<T>()).and_then(|v| v.downcast_ref())
        }
    }

    let mut extensions = Extensions::new();
    extensions.insert("My Extension");
    extensions.insert(5u32);

    let res = Response {
        inner: Ok(ResponseInner { extensions }),
    };

    let extensions_ref = res.extensions_ref().unwrap();
    assert_eq!(extensions_ref.get::<&'static str>(), Some(&"My Extension"));
    assert_eq!(extensions_ref.get::<u32>(), Some(&5u32));
}

#[test]
fn test_extensions_ref_with_error() {
    use http::Response;

    struct ResponseInner {
        extensions: Extensions,
    }

    let res = Response {
        inner: Err("Error initializing response"),
    };

    assert!(res.extensions_ref().is_none());
}

