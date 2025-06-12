// Answer 0

#[test]
fn test_from_iter_with_valid_input() {
    use http::header::{HeaderMap, HeaderName};
    
    let headers: Vec<(HeaderName, &str)> = vec![
        (HeaderName::from_static("content-type"), "application/json"),
        (HeaderName::from_static("user-agent"), "test-agent"),
        (HeaderName::from_static("accept"), "text/html"),
    ];

    let map: HeaderMap = HeaderMap::from_iter(headers);

    assert_eq!(map.get("content-type").unwrap(), "application/json");
    assert_eq!(map.get("user-agent").unwrap(), "test-agent");
    assert_eq!(map.get("accept").unwrap(), "text/html");
}

#[test]
#[should_panic]
fn test_from_iter_with_empty_iterator() {
    use http::header::{HeaderMap, HeaderName};
    
    let headers: Vec<(HeaderName, &str)> = vec![];

    let map: HeaderMap = HeaderMap::from_iter(headers);

    assert!(map.is_empty());
}

#[test]
fn test_from_iter_with_repeated_headers() {
    use http::header::{HeaderMap, HeaderName};
    
    let headers: Vec<(HeaderName, &str)> = vec![
        (HeaderName::from_static("content-type"), "application/json"),
        (HeaderName::from_static("content-type"), "text/html"), // repeated header
    ];

    let map: HeaderMap = HeaderMap::from_iter(headers);

    assert_eq!(map.get("content-type").unwrap(), "text/html"); // should take the last value
}

#[test]
fn test_from_iter_with_special_characters() {
    use http::header::{HeaderMap, HeaderName};
    
    let headers: Vec<(HeaderName, &str)> = vec![
        (HeaderName::from_static("x-custom-header"), "value@123"),
        (HeaderName::from_static("x_another-header"), "val#ue$%&*"),
    ];

    let map: HeaderMap = HeaderMap::from_iter(headers);

    assert_eq!(map.get("x-custom-header").unwrap(), "value@123");
    assert_eq!(map.get("x_another-header").unwrap(), "val#ue$%&*");
}

