// Answer 0

#[derive(Default, Debug, PartialEq)]
struct Parts {
    status: StatusCode,
}

#[derive(Default)]
struct Response<T> {
    head: Parts,
    body: T,
}

#[derive(Debug, PartialEq)]
enum StatusCode {
    OK,
    NotFound,
}

#[test]
fn test_into_parts_default_response() {
    let response: Response<()> = Response::default();
    let (parts, body) = response.into_parts();
    assert_eq!(parts.status, StatusCode::OK);
    assert_eq!(body, ());
}

#[test]
fn test_into_parts_custom_body() {
    let response: Response<String> = Response {
        head: Parts { status: StatusCode::OK },
        body: "Hello, world!".to_string(),
    };
    let (parts, body) = response.into_parts();
    assert_eq!(parts.status, StatusCode::OK);
    assert_eq!(body, "Hello, world!");
}

#[test]
fn test_into_parts_not_found_response() {
    let response: Response<()> = Response {
        head: Parts { status: StatusCode::NotFound },
        body: (),
    };
    let (parts, body) = response.into_parts();
    assert_eq!(parts.status, StatusCode::NotFound);
    assert_eq!(body, ());
}

#[test]
fn test_into_parts_empty_body() {
    let response: Response<Vec<u8>> = Response {
        head: Parts { status: StatusCode::OK },
        body: vec![],
    };
    let (parts, body) = response.into_parts();
    assert_eq!(parts.status, StatusCode::OK);
    assert_eq!(body, vec![]);
}

