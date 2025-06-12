// Answer 0

#[test]
fn test_header_with_valid_string_key_and_value() {
    use http::header::HeaderValue;
    use http::{Response, Builder};

    let response = Response::builder()
        .header("Content-Type", "text/html")
        .header("X-Custom-Foo", "bar")
        .header("content-length", 0)
        .body(())
        .unwrap();

    assert_eq!(response.headers().get("Content-Type").unwrap(), &HeaderValue::from_static("text/html"));
    assert_eq!(response.headers().get("X-Custom-Foo").unwrap(), &HeaderValue::from_static("bar"));
    assert_eq!(response.headers().get("content-length").unwrap().to_str().unwrap(), "0");
}

#[test]
#[should_panic]
fn test_header_with_invalid_header_name() {
    use http::{Response, Builder};

    let _ = Response::builder()
        .header("Invalid-Header-Name-", "value")
        .header("Another-Header", "value")
        .body(())
        .unwrap();
}

#[test]
fn test_header_with_empty_value() {
    use http::header::HeaderValue;
    use http::{Response, Builder};

    let response = Response::builder()
        .header("Content-Type", "")
        .body(())
        .unwrap();

    assert_eq!(response.headers().get("Content-Type").unwrap(), &HeaderValue::from_static(""));
}

#[test]
fn test_header_with_numeric_value_as_string() {
    use http::header::HeaderValue;
    use http::{Response, Builder};

    let response = Response::builder()
        .header("Content-Length", "123")
        .body(())
        .unwrap();

    assert_eq!(response.headers().get("Content-Length").unwrap().to_str().unwrap(), "123");
}

#[test]
fn test_header_with_special_characters() {
    use http::header::HeaderValue;
    use http::{Response, Builder};

    let response = Response::builder()
        .header("X-Custom-Header", "value!@#$%^&*()")
        .body(())
        .unwrap();

    assert_eq!(response.headers().get("X-Custom-Header").unwrap(), &HeaderValue::from_static("value!@#$%^&*()"));
}

#[test]
fn test_header_with_mixed_case_key() {
    use http::header::HeaderValue;
    use http::{Response, Builder};

    let response = Response::builder()
        .header("Content-Type", "text/plain")
        .body(())
        .unwrap();

    assert_eq!(response.headers().get("content-type").unwrap(), &HeaderValue::from_static("text/plain"));
}

