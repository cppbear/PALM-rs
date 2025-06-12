// Answer 0

#[test]
fn test_query_none() {
    struct PathAndQuery {
        query: usize,
        data: String,
    }

    impl std::str::FromStr for PathAndQuery {
        type Err = std::convert::Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let query_index = s.find('?').unwrap_or_else(|| s.len());
            let data = s.to_string();
            let query = if query_index < data.len() { query_index } else { NONE };
            Ok(PathAndQuery { query, data })
        }
    }

    const NONE: usize = usize::MAX;

    let path_and_query: PathAndQuery = "/hello/world".parse().unwrap();
    assert!(path_and_query.query() == None);
}

