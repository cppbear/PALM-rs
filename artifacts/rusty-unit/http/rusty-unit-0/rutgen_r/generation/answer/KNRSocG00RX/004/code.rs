// Answer 0

#[test]
fn test_path_with_non_empty_data_and_query() {
    struct PathAndQuery {
        data: String,
        query: usize,
    }

    impl PathAndQuery {
        const NONE: usize = usize::MAX;

        pub fn path(&self) -> &str {
            let ret = if self.query == Self::NONE {
                &self.data[..]
            } else {
                &self.data[..self.query]
            };

            if ret.is_empty() {
                return "/";
            }

            ret
        }
    }

    let path_and_query = PathAndQuery {
        data: "/hello/world?key=value".to_string(),
        query: 14, // Length of "/hello/world"
    };

    assert_eq!(path_and_query.path(), "/hello/world");
}

#[test]
fn test_path_with_full_path_and_query() {
    struct PathAndQuery {
        data: String,
        query: usize,
    }

    impl PathAndQuery {
        const NONE: usize = usize::MAX;

        pub fn path(&self) -> &str {
            let ret = if self.query == Self::NONE {
                &self.data[..]
            } else {
                &self.data[..self.query]
            };

            if ret.is_empty() {
                return "/";
            }

            ret
        }
    }

    let path_and_query = PathAndQuery {
        data: "/hello/world?key=value".to_string(),
        query: path_and_query.data.len(), // Full length to include the entire data part
    };

    assert_eq!(path_and_query.path(), "/hello/world?key=value");
}

#[test]
fn test_path_with_no_query() {
    struct PathAndQuery {
        data: String,
        query: usize,
    }

    impl PathAndQuery {
        const NONE: usize = usize::MAX;

        pub fn path(&self) -> &str {
            let ret = if self.query == Self::NONE {
                &self.data[..]
            } else {
                &self.data[..self.query]
            };

            if ret.is_empty() {
                return "/";
            }

            ret
        }
    }

    let path_and_query = PathAndQuery {
        data: "/hello/world".to_string(),
        query: PathAndQuery::NONE,
    };

    assert_eq!(path_and_query.path(), "/hello/world");
}

#[test]
#[should_panic]
fn test_path_with_invalid_query() {
    struct PathAndQuery {
        data: String,
        query: usize,
    }

    impl PathAndQuery {
        const NONE: usize = usize::MAX;

        pub fn path(&self) -> &str {
            let ret = if self.query == Self::NONE {
                &self.data[..]
            } else {
                &self.data[..self.query]
            };

            if ret.is_empty() {
                return "/";
            }

            ret
        }
    }

    let path_and_query = PathAndQuery {
        data: "/hello/world".to_string(),
        query: 15, // Invalid query length, greater than the data length
    };

    // This should panic due to out-of-bounds access
    let _ = path_and_query.path();
}

