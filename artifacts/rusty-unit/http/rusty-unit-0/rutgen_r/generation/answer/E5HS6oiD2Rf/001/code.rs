// Answer 0

#[test]
fn test_from_parts_authority_missing() {
    struct Parts {
        scheme: Option<Scheme>,
        authority: Option<Authority>,
        path_and_query: Option<PathAndQuery>,
    }

    struct Scheme {
        inner: Scheme2,
    }

    enum Scheme2 {
        None,
        Http,
        Https,
    }

    struct Authority(String);
    
    struct PathAndQuery(String);

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    #[derive(Debug)]
    struct InvalidUriParts;

    #[derive(Debug)]
    enum ErrorKind {
        AuthorityMissing,
        PathAndQueryMissing,
        SchemeMissing,
    }

    impl From<ErrorKind> for InvalidUriParts {
        fn from(_: ErrorKind) -> Self {
            InvalidUriParts
        }
    }

    let parts = Parts {
        scheme: Some(Scheme { inner: Scheme2::Http }),
        authority: None,
        path_and_query: Some(PathAndQuery("/foo".to_string())),
    };

    let result = from_parts(parts);
    assert_eq!(result, Err(ErrorKind::AuthorityMissing.into()));
}

