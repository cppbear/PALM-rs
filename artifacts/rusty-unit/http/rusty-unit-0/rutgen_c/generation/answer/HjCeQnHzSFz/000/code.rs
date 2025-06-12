// Answer 0

#[test]
fn test_builder_empty() {
    let uri = Uri::builder().build().unwrap();
    assert_eq!(uri.scheme(), Some(&Scheme { inner: Scheme2::None }));
    assert_eq!(uri.authority(), Some(&Authority::empty()));
    assert_eq!(uri.path_and_query(), Some(&PathAndQuery::empty()));
}

#[test]
fn test_builder_with_scheme() {
    #[derive(Debug)]
    struct TestScheme;
    
    impl TryInto<Scheme> for TestScheme {
        type Error = crate::Error;
        
        fn try_into(self) -> Result<Scheme, Self::Error> {
            Ok(Scheme { inner: Scheme2::Http }) // Assume Http is a valid variant
        }
    }

    let uri = Uri::builder()
        .scheme(TestScheme)
        .build()
        .unwrap();
    
    assert_eq!(uri.scheme(), Some(&Scheme { inner: Scheme2::Http }));
}

#[test]
fn test_builder_with_authority() {
    #[derive(Debug)]
    struct TestAuthority;

    impl TryInto<Authority> for TestAuthority {
        type Error = crate::Error;

        fn try_into(self) -> Result<Authority, Self::Error> {
            Ok(Authority::from_str("example.com").unwrap())
        }
    }

    let uri = Uri::builder()
        .authority(TestAuthority)
        .build()
        .unwrap();

    assert_eq!(uri.authority(), Some(&Authority::from_str("example.com").unwrap()));
}

#[test]
fn test_builder_with_path_and_query() {
    #[derive(Debug)]
    struct TestPathAndQuery;

    impl TryInto<PathAndQuery> for TestPathAndQuery {
        type Error = crate::Error;

        fn try_into(self) -> Result<PathAndQuery, Self::Error> {
            Ok(PathAndQuery::from_str("/path?query=1").unwrap())
        }
    }

    let uri = Uri::builder()
        .path_and_query(TestPathAndQuery)
        .build()
        .unwrap();

    assert_eq!(uri.path_and_query(), Some(&PathAndQuery::from_str("/path?query=1").unwrap()));
}

#[test]
#[should_panic]
fn test_builder_invalid_uri() {
    // Testing for panic on invalid combination
    let uri = Uri::builder()
        .authority("example.com")
        .build(); // This should panic because no scheme is provided
}

