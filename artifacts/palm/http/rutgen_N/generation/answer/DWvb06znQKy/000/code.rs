// Answer 0

#[test]
fn test_query_with_valid_query_string() {
    struct PathAndQuery {
        query: usize,
        data: String,
    }
    
    impl PathAndQuery {
        fn parse(input: &str) -> Result<Self, &'static str> {
            if let Some((path, query)) = input.split_once('?') {
                Ok(PathAndQuery {
                    query: path.len(),
                    data: format!("{}?{}", path, query),
                })
            } else {
                Ok(PathAndQuery {
                    query: NONE,
                    data: input.to_string(),
                })
            }
        }

        fn query(&self) -> Option<&str> {
            const NONE: usize = usize::MAX;
            if self.query == NONE {
                None
            } else {
                let i = self.query + 1;
                Some(&self.data[i..])
            }
        }
    }
    
    const NONE: usize = usize::MAX;

    let path_and_query: PathAndQuery = PathAndQuery::parse("/hello/world?key=value&foo=bar").unwrap();
    assert_eq!(path_and_query.query(), Some("key=value&foo=bar"));
}

#[test]
fn test_query_without_query_string() {
    struct PathAndQuery {
        query: usize,
        data: String,
    }

    impl PathAndQuery {
        fn parse(input: &str) -> Result<Self, &'static str> {
            if let Some((path, query)) = input.split_once('?') {
                Ok(PathAndQuery {
                    query: path.len(),
                    data: format!("{}?{}", path, query),
                })
            } else {
                Ok(PathAndQuery {
                    query: NONE,
                    data: input.to_string(),
                })
            }
        }

        fn query(&self) -> Option<&str> {
            const NONE: usize = usize::MAX;
            if self.query == NONE {
                None
            } else {
                let i = self.query + 1;
                Some(&self.data[i..])
            }
        }
    }

    const NONE: usize = usize::MAX;

    let path_and_query: PathAndQuery = PathAndQuery::parse("/hello/world").unwrap();
    assert!(path_and_query.query().is_none());
}

