// Answer 0

#[test]
fn test_as_str_empty_path() {
    struct PathAndQuery {
        data: String,
    }

    let path_and_query = PathAndQuery { data: String::new() };
    assert_eq!(path_and_query.as_str(), "/");
}

#[test]
fn test_as_str_non_empty_path() {
    struct PathAndQuery {
        data: String,
    }

    let path_and_query = PathAndQuery { data: "/hello/world".to_string() };
    assert_eq!(path_and_query.as_str(), "/hello/world");
}

#[test]
fn test_as_str_with_query() {
    struct PathAndQuery {
        data: String,
    }

    let path_and_query = PathAndQuery { data: "/hello/world?key=value&foo=bar".to_string() };
    assert_eq!(path_and_query.as_str(), "/hello/world?key=value&foo=bar");
}

