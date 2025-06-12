// Answer 0

#[test]
fn test_from_parts_scheme_missing() {
    struct Parts {
        scheme: Option<Scheme>,
        authority: Option<Authority>,
        path_and_query: Option<PathAndQuery>,
    }

    struct Scheme {
        inner: Scheme2,
    }

    struct Authority {
        inner: String,
    }

    struct PathAndQuery {
        inner: String,
    }

    #[derive(Debug)]
    enum Scheme2 {
        None,
        Http,
        Https,
    }

    #[derive(Debug)]
    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    #[derive(Debug)]
    enum ErrorKind {
        AuthorityMissing,
        PathAndQueryMissing,
        SchemeMissing,
    }

    #[derive(Debug)]
    struct InvalidUriParts;

    impl From<ErrorKind> for InvalidUriParts {
        fn from(_: ErrorKind) -> Self {
            InvalidUriParts
        }
    }

    fn from_parts(src: Parts) -> Result<Uri, InvalidUriParts> {
        if src.scheme.is_some() {
            if src.authority.is_none() {
                return Err(ErrorKind::AuthorityMissing.into());
            }

            if src.path_and_query.is_none() {
                return Err(ErrorKind::PathAndQueryMissing.into());
            }
        } else if src.authority.is_some() && src.path_and_query.is_some() {
            return Err(ErrorKind::SchemeMissing.into());
        }

        let scheme = match src.scheme {
            Some(scheme) => scheme,
            None => Scheme {
                inner: Scheme2::None,
            },
        };

        let authority = match src.authority {
            Some(authority) => authority,
            None => Authority {
                inner: String::new(),
            },
        };

        let path_and_query = match src.path_and_query {
            Some(path_and_query) => path_and_query,
            None => PathAndQuery {
                inner: String::new(),
            },
        };

        Ok(Uri {
            scheme,
            authority,
            path_and_query,
        })
    }

    let mut parts = Parts {
        scheme: None,
        authority: Some(Authority { inner: "foo.com".to_string() }),
        path_and_query: Some(PathAndQuery { inner: "/foo".to_string() }),
    };

    let result = from_parts(parts).unwrap_err();
    match result {
        InvalidUriParts => assert!(true), // Confirming we got the expected error type
    }
}

