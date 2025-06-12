// Answer 0

#[test]
fn test_headers_mut_success() {
    struct TestMethod;
    struct TestUri;
    struct TestVersion;
    struct TestExtensions;

    let builder = Builder::new()
        .method(TestMethod {})
        .uri(TestUri {})
        .version(TestVersion {})
        .extension(TestExtensions {});

    let mut req = builder;
    let headers = req.headers_mut();
    assert!(headers.is_some());

    let headers = headers.unwrap();
    headers.entries.push(Bucket::new("Accept", HeaderValue::from_static("text/html")));
    headers.entries.push(Bucket::new("X-Custom-Foo", HeaderValue::from_static("bar")));
}

#[test]
fn test_headers_mut_failure() {
    struct TestError;
    struct TestMethod;
    struct TestUri;
    struct TestVersion;

    let invalid_builder = Builder {
        inner: Err(Error { inner: TestError }),
    };

    let req = invalid_builder;
    let headers = req.headers_mut();
    assert!(headers.is_none());
}

