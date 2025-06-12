// Answer 0

#[test]
fn test_headers_empty() {
    struct Response<T> {
        head: Head<T>,
    }

    struct Head<T> {
        headers: HeaderMap<HeaderValue>,
    }

    impl<T> Default for Response<T> {
        fn default() -> Self {
            Response {
                head: Head {
                    headers: HeaderMap::new(),
                },
            }
        }
    }

    use http::HeaderMap;
    use http::header::HeaderValue;

    let response: Response<()> = Response::default();
    assert!(response.headers().is_empty());
}

#[test]
fn test_headers_non_empty() {
    struct Response<T> {
        head: Head<T>,
    }

    struct Head<T> {
        headers: HeaderMap<HeaderValue>,
    }

    impl<T> Default for Response<T> {
        fn default() -> Self {
            Response {
                head: Head {
                    headers: HeaderMap::new(),
                },
            }
        }
    }

    use http::HeaderMap;
    use http::header::HeaderValue;

    let mut response: Response<()> = Response::default();
    response.head.headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    assert!(!response.headers().is_empty());
    assert_eq!(response.headers().get("Content-Type").unwrap(), "application/json");
}

