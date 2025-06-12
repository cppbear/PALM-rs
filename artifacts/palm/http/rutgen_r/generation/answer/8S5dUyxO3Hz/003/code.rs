// Answer 0

#[test]
fn test_path_and_query_with_non_empty_scheme_and_authority() {
    struct Scheme {
        inner: Option<String>,
    }
    
    struct Authority {
        data: String,
    }
    
    struct PathAndQuery {
        path: String,
        query: String,
    }
    
    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    let uri = Uri {
        scheme: Scheme { inner: Some(String::from("http")) },
        authority: Authority { data: String::from("example.com") },
        path_and_query: PathAndQuery { path: String::from("/path"), query: String::from("?query=value") },
    };

    let result = uri.path_and_query();
    assert!(result.is_some());
    assert_eq!(result.unwrap().path, "/path");
    assert_eq!(result.unwrap().query, "?query=value");
}

#[test]
fn test_path_and_query_with_non_empty_scheme_and_empty_authority() {
    struct Scheme {
        inner: Option<String>,
    }
    
    struct Authority {
        data: String,
    }
    
    struct PathAndQuery {
        path: String,
        query: String,
    }
    
    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    let uri = Uri {
        scheme: Scheme { inner: Some(String::from("https")) },
        authority: Authority { data: String::from("") },
        path_and_query: PathAndQuery { path: String::from("/another_path"), query: String::from("?another_query=value") },
    };

    let result = uri.path_and_query();
    assert!(result.is_some());
    assert_eq!(result.unwrap().path, "/another_path");
    assert_eq!(result.unwrap().query, "?another_query=value");
}

