// Answer 0

#[test]
fn test_path_and_query_with_query() {
    struct PathAndQuery {
        data: String,
    }

    impl std::str::FromStr for PathAndQuery {
        type Err = std::string::FromUtf8Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(PathAndQuery {
                data: s.to_string(),
            })
        }
    }

    let path_and_query: PathAndQuery = "/hello/world?key=value&foo=bar".parse().unwrap();
    assert_eq!(path_and_query.as_str(), "/hello/world?key=value&foo=bar");
}

#[test]
fn test_path_and_query_without_query() {
    struct PathAndQuery {
        data: String,
    }

    impl std::str::FromStr for PathAndQuery {
        type Err = std::string::FromUtf8Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(PathAndQuery {
                data: s.to_string(),
            })
        }
    }

    let path_and_query: PathAndQuery = "/hello/world".parse().unwrap();
    assert_eq!(path_and_query.as_str(), "/hello/world");
}

#[test]
fn test_path_and_query_empty_string() {
    struct PathAndQuery {
        data: String,
    }

    impl std::str::FromStr for PathAndQuery {
        type Err = std::string::FromUtf8Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(PathAndQuery {
                data: s.to_string(),
            })
        }
    }

    let path_and_query: PathAndQuery = "".parse().unwrap();
    assert_eq!(path_and_query.as_str(), "/");
}

