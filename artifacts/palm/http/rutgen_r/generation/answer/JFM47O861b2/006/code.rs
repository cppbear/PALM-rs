// Answer 0

#[test]
fn test_from_shared_empty_authority() {
    use bytes::Bytes;

    struct Authority;

    impl Authority {
        fn empty() -> Self {
            Authority
        }

        fn from_shared(s: Bytes) -> Result<Self, ()> {
            if s[0] != b'/' && s[0] != b'*' {
                Ok(Authority)
            } else {
                Err(())
            }
        }
    }

    struct PathAndQuery;

    impl PathAndQuery {
        fn empty() -> Self {
            PathAndQuery
        }

        fn from_shared(_s: Bytes) -> Result<Self, ()> {
            Ok(PathAndQuery)
        }

        fn slash() -> Self {
            PathAndQuery
        }

        fn star() -> Self {
            PathAndQuery
        }
    }

    struct Scheme;

    impl Scheme {
        fn empty() -> Self {
            Scheme
        }
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    const MAX_LEN: usize = 10;

    fn from_shared(s: Bytes) -> Result<Uri, ()> {
        if s.len() > MAX_LEN {
            return Err(());
        }

        match s.len() {
            0 => return Err(()),
            1 => match s[0] {
                b'/' => return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::slash() }),
                b'*' => return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::star() }),
                _ => {
                    let authority = Authority::from_shared(s)?;
                    return Ok(Uri { scheme: Scheme::empty(), authority, path_and_query: PathAndQuery::empty() });
                }
            },
            _ => {}
        }

        if s[0] == b'/' {
            return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::from_shared(s)? });
        }

        Err(())
    }

    let bytes = Bytes::from_static(b"a");
    let result = from_shared(bytes).unwrap();
    assert_eq!(result.scheme, Scheme::empty());
    assert_eq!(result.authority, Authority);
    assert_eq!(result.path_and_query, PathAndQuery::empty());
}

#[test]
fn test_from_shared_slash() {
    use bytes::Bytes;

    struct Authority;

    impl Authority {
        fn empty() -> Self {
            Authority
        }
        
        fn from_shared(s: Bytes) -> Result<Self, ()> {
            if s[0] != b'/' && s[0] != b'*' {
                Ok(Authority)
            } else {
                Err(())
            }
        }
    }

    struct PathAndQuery;

    impl PathAndQuery {
        fn empty() -> Self {
            PathAndQuery
        }

        fn from_shared(_s: Bytes) -> Result<Self, ()> {
            Ok(PathAndQuery)
        }

        fn slash() -> Self {
            PathAndQuery
        }

        fn star() -> Self {
            PathAndQuery
        }
    }

    struct Scheme;

    impl Scheme {
        fn empty() -> Self {
            Scheme
        }
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    const MAX_LEN: usize = 10;

    fn from_shared(s: Bytes) -> Result<Uri, ()> {
        if s.len() > MAX_LEN {
            return Err(());
        }

        match s.len() {
            0 => return Err(()),
            1 => match s[0] {
                b'/' => return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::slash() }),
                b'*' => return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::star() }),
                _ => {
                    let authority = Authority::from_shared(s)?;
                    return Ok(Uri { scheme: Scheme::empty(), authority, path_and_query: PathAndQuery::empty() });
                }
            },
            _ => {}
        }

        if s[0] == b'/' {
            return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::from_shared(s)? });
        }

        Err(())
    }

    let bytes = Bytes::from_static(b"/");
    let result = from_shared(bytes).unwrap();
    assert_eq!(result.scheme, Scheme::empty());
    assert_eq!(result.authority, Authority::empty());
    assert_eq!(result.path_and_query, PathAndQuery::slash());
}

#[test]
fn test_from_shared_star() {
    use bytes::Bytes;

    struct Authority;

    impl Authority {
        fn empty() -> Self {
            Authority
        }
        
        fn from_shared(s: Bytes) -> Result<Self, ()> {
            if s[0] != b'/' && s[0] != b'*' {
                Ok(Authority)
            } else {
                Err(())
            }
        }
    }

    struct PathAndQuery;

    impl PathAndQuery {
        fn empty() -> Self {
            PathAndQuery
        }

        fn from_shared(_s: Bytes) -> Result<Self, ()> {
            Ok(PathAndQuery)
        }

        fn slash() -> Self {
            PathAndQuery
        }

        fn star() -> Self {
            PathAndQuery
        }
    }

    struct Scheme;

    impl Scheme {
        fn empty() -> Self {
            Scheme
        }
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    const MAX_LEN: usize = 10;

    fn from_shared(s: Bytes) -> Result<Uri, ()> {
        if s.len() > MAX_LEN {
            return Err(());
        }

        match s.len() {
            0 => return Err(()),
            1 => match s[0] {
                b'/' => return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::slash() }),
                b'*' => return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::star() }),
                _ => {
                    let authority = Authority::from_shared(s)?;
                    return Ok(Uri { scheme: Scheme::empty(), authority, path_and_query: PathAndQuery::empty() });
                }
            },
            _ => {}
        }

        if s[0] == b'/' {
            return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::from_shared(s)? });
        }

        Err(())
    }

    let bytes = Bytes::from_static(b"*");
    let result = from_shared(bytes).unwrap();
    assert_eq!(result.scheme, Scheme::empty());
    assert_eq!(result.authority, Authority::empty());
    assert_eq!(result.path_and_query, PathAndQuery::star());
}

#[test]
fn test_from_shared_invalid_auth() {
    use bytes::Bytes;

    struct Authority;

    impl Authority {
        fn empty() -> Self {
            Authority
        }
        
        fn from_shared(s: Bytes) -> Result<Self, ()> {
            if s[0] == b'a' { Ok(Authority) } else { Err(()) }
        }
    }

    struct PathAndQuery;

    impl PathAndQuery {
        fn empty() -> Self {
            PathAndQuery
        }

        fn from_shared(_s: Bytes) -> Result<Self, ()> {
            Ok(PathAndQuery)
        }

        fn slash() -> Self {
            PathAndQuery
        }

        fn star() -> Self {
            PathAndQuery
        }
    }

    struct Scheme;

    impl Scheme {
        fn empty() -> Self {
            Scheme
        }
    }

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    const MAX_LEN: usize = 10;

    fn from_shared(s: Bytes) -> Result<Uri, ()> {
        if s.len() > MAX_LEN {
            return Err(());
        }

        match s.len() {
            0 => return Err(()),
            1 => match s[0] {
                b'/' => return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::slash() }),
                b'*' => return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::star() }),
                _ => {
                    let authority = Authority::from_shared(s)?;
                    return Ok(Uri { scheme: Scheme::empty(), authority, path_and_query: PathAndQuery::empty() });
                }
            },
            _ => {}
        }

        if s[0] == b'/' {
            return Ok(Uri { scheme: Scheme::empty(), authority: Authority::empty(), path_and_query: PathAndQuery::from_shared(s)? });
        }

        Err(())
    }

    let bytes = Bytes::from_static(b"b");
    let result = from_shared(bytes);
    assert!(result.is_err());
}

