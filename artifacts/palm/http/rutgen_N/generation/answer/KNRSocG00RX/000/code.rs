// Answer 0

#[test]
fn test_path_with_non_empty_path_and_no_query() {
    struct PathAndQuery {
        data: String,
        query: Option<usize>,
    }

    impl PathAndQuery {
        fn path(&self) -> &str {
            let ret = if self.query.is_none() {
                &self.data[..]
            } else {
                &self.data[..self.query.unwrap()]
            };

            if ret.is_empty() {
                return "/";
            }

            ret
        }
    }

    let path_and_query = PathAndQuery {
        data: "/hello/world".to_string(),
        query: None,
    };
    
    assert_eq!(path_and_query.path(), "/hello/world");
}

#[test]
fn test_path_with_empty_path() {
    struct PathAndQuery {
        data: String,
        query: Option<usize>,
    }

    impl PathAndQuery {
        fn path(&self) -> &str {
            let ret = if self.query.is_none() {
                &self.data[..]
            } else {
                &self.data[..self.query.unwrap()]
            };

            if ret.is_empty() {
                return "/";
            }

            ret
        }
    }

    let path_and_query = PathAndQuery {
        data: "".to_string(),
        query: None,
    };
    
    assert_eq!(path_and_query.path(), "/");
}

#[test]
fn test_path_with_path_and_query() {
    struct PathAndQuery {
        data: String,
        query: Option<usize>,
    }

    impl PathAndQuery {
        fn path(&self) -> &str {
            let ret = if self.query.is_none() {
                &self.data[..]
            } else {
                &self.data[..self.query.unwrap()]
            };

            if ret.is_empty() {
                return "/";
            }

            ret
        }
    }

    let path_and_query = PathAndQuery {
        data: "/hello/world?key=value".to_string(),
        query: Some(13),
    };
    
    assert_eq!(path_and_query.path(), "/hello/world");
}

#[test]
fn test_path_with_wildcard() {
    struct PathAndQuery {
        data: String,
        query: Option<usize>,
    }

    impl PathAndQuery {
        fn path(&self) -> &str {
            let ret = if self.query.is_none() {
                &self.data[..]
            } else {
                &self.data[..self.query.unwrap()]
            };

            if ret.is_empty() {
                return "/";
            }

            ret
        }
    }

    let path_and_query = PathAndQuery {
        data: "*".to_string(),
        query: None,
    };
    
    assert_eq!(path_and_query.path(), "*");
}

