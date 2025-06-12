// Answer 0

#[test]
fn test_into_parts() {
    struct Method {
        name: &'static str,
    }

    struct Parts {
        method: Method,
    }

    struct Request<T> {
        head: Parts,
        body: T,
    }

    impl<T> Request<T> {
        pub fn new(body: T) -> Self {
            Request {
                head: Parts {
                    method: Method { name: "GET" },
                },
                body,
            }
        }

        pub fn into_parts(self) -> (Parts, T) {
            (self.head, self.body)
        }
    }

    let request = Request::new(());
    let (parts, body) = request.into_parts();
    assert_eq!(parts.method.name, "GET");
}

