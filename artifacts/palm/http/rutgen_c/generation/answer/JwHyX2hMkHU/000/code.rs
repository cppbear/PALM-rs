// Answer 0

#[test]
fn test_map_function_with_string_body() {
    struct MockResponse {
        body: String,
        head: Parts,
    }

    let response = Response::new("some string".to_string());
    let mapped_response: Response<Vec<u8>> = response.map(|b| {
        assert_eq!(b, "some string");
        b.as_bytes().to_vec()
    });
    assert_eq!(mapped_response.body(), &b"some string"[..]);
}

#[test]
fn test_map_function_with_integer_body() {
    struct MockResponse {
        body: i32,
        head: Parts,
    }

    let response = Response::new(42);
    let mapped_response: Response<String> = response.map(|b| {
        assert_eq!(b, 42);
        b.to_string()
    });
    assert_eq!(mapped_response.body(), "42");
}

#[test]
fn test_map_function_with_empty_string_body() {
    struct MockResponse {
        body: String,
        head: Parts,
    }

    let response = Response::new("".to_string());
    let mapped_response: Response<Vec<u8>> = response.map(|b| {
        assert_eq!(b, "");
        b.as_bytes().to_vec()
    });
    assert_eq!(mapped_response.body(), &b""[..]);
}

