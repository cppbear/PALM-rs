// Answer 0

#[test]
fn test_path_and_query_with_none_scheme_and_empty_authority() {
    struct Scheme {
        inner: Option<String>,
    }

    struct Authority {
        data: String,
    }

    struct PathAndQuery {
        value: String,
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    impl Uri {
        pub fn path_and_query(&self) -> Option<&PathAndQuery> {
            if !self.scheme.inner.is_none() || self.authority.data.is_empty() {
                Some(&self.path_and_query)
            } else {
                None
            }
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: None },
        authority: Authority { data: String::new() },
        path_and_query: PathAndQuery { value: String::from("/path?query=true") },
    };

    let result = uri.path_and_query();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "/path?query=true");
}

#[test]
fn test_path_and_query_with_none_scheme_and_non_empty_authority() {
    struct Scheme {
        inner: Option<String>,
    }

    struct Authority {
        data: String,
    }

    struct PathAndQuery {
        value: String,
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    impl Uri {
        pub fn path_and_query(&self) -> Option<&PathAndQuery> {
            if !self.scheme.inner.is_none() || self.authority.data.is_empty() {
                Some(&self.path_and_query)
            } else {
                None
            }
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: None },
        authority: Authority { data: String::from("authority") },
        path_and_query: PathAndQuery { value: String::from("/path?query=true") },
    };

    let result = uri.path_and_query();
    assert!(result.is_none());
}

#[test]
fn test_path_and_query_with_some_scheme_and_empty_authority() {
    struct Scheme {
        inner: Option<String>,
    }

    struct Authority {
        data: String,
    }

    struct PathAndQuery {
        value: String,
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    impl Uri {
        pub fn path_and_query(&self) -> Option<&PathAndQuery> {
            if !self.scheme.inner.is_none() || self.authority.data.is_empty() {
                Some(&self.path_and_query)
            } else {
                None
            }
        }
    }
    
    let uri = Uri {
        scheme: Scheme { inner: Some(String::from("http")) },
        authority: Authority { data: String::new() },
        path_and_query: PathAndQuery { value: String::from("/path?query=true") },
    };

    let result = uri.path_and_query();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "/path?query=true");
}

