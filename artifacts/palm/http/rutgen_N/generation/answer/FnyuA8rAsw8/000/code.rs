// Answer 0

#[test]
fn test_headers_mut_success() {
    use http::*;
    use http::header::{HeaderValue, HeaderMap};
    use http::response::Response;

    // Initialize the response builder
    let mut res = Response::builder();

    // Access the mutable headers and insert values
    {
        let headers = res.headers_mut().unwrap();
        headers.insert("Accept", HeaderValue::from_static("text/html"));
        headers.insert("X-Custom-Foo", HeaderValue::from_static("bar"));
    }

    // Verify the headers were set correctly
    let headers = res.headers_ref().unwrap();
    assert_eq!(headers["Accept"], "text/html");
    assert_eq!(headers["X-Custom-Foo"], "bar");
}

#[test]
fn test_headers_mut_error() {
    use http::*;
    use http::header::{HeaderValue};
    use http::response::Response;

    // Create a response that simulates an error condition
    let res: Response<()> = Response::builder().body(()).unwrap_err();

    // Verify that the headers_mut method returns None
    let headers = res.headers_mut();
    assert!(headers.is_none());
}

