// Answer 0

#[test]
fn test_builder_success_with_valid_parts() {
    struct DummyScheme;
    struct DummyAuthority;
    struct DummyPathAndQuery;

    impl TryInto<Scheme> for DummyScheme {
        type Error = crate::Error; // Assuming a valid implementation exists

        fn try_into(self) -> Result<Scheme, Self::Error> {
            Ok(Scheme { inner: Scheme2::Http }) // Example valid Scheme
        }
    }

    impl TryInto<Authority> for DummyAuthority {
        type Error = crate::Error; // Assuming a valid implementation exists

        fn try_into(self) -> Result<Authority, Self::Error> {
            Ok(Authority::from_static("localhost:80")) // Example valid Authority
        }
    }

    impl TryInto<PathAndQuery> for DummyPathAndQuery {
        type Error = crate::Error; // Assuming a valid implementation exists

        fn try_into(self) -> Result<PathAndQuery, Self::Error> {
            Ok(PathAndQuery::from_static("/index.html")) // Example valid PathAndQuery
        }
    }

    let uri = Builder::new()
        .scheme(DummyScheme)
        .authority(DummyAuthority)
        .path_and_query(DummyPathAndQuery)
        .build()
        .unwrap();

    assert!(uri.scheme().is_some());
    assert!(uri.authority().is_some());
    assert!(uri.path_and_query().is_some());
}

#[test]
#[should_panic]
fn test_builder_with_missing_authority() {
    struct DummyScheme;

    impl TryInto<Scheme> for DummyScheme {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            Ok(Scheme { inner: Scheme2::Http }) // Example valid Scheme
        }
    }
    
    let uri = Builder::new()
        .scheme(DummyScheme)
        .path_and_query(DummyPathAndQuery) // Authority is missing
        .build()
        .unwrap();
}

#[test]
#[should_panic]
fn test_builder_with_missing_path_and_query() {
    struct DummyScheme;
    struct DummyAuthority;

    impl TryInto<Scheme> for DummyScheme {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            Ok(Scheme { inner: Scheme2::Http }) // Example valid Scheme
        }
    }

    impl TryInto<Authority> for DummyAuthority {
        type Error = crate::Error;

        fn try_into(self) -> Result<Authority, Self::Error> {
            Ok(Authority::from_static("localhost:80")) // Example valid Authority
        }
    }

    let uri = Builder::new()
        .scheme(DummyScheme)
        .authority(DummyAuthority)
        .build()
        .unwrap(); // PathAndQuery is missing, expected to panic.
}

#[test]
#[should_panic]
fn test_builder_with_invalid_scheme() {
    struct InvalidScheme;

    impl TryInto<Scheme> for InvalidScheme {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            Err(crate::Error::new("Invalid scheme")) // Simulating invalid scheme
        }
    }

    let uri = Builder::new()
        .scheme(InvalidScheme)
        .authority(DummyAuthority)
        .path_and_query(DummyPathAndQuery)
        .build() // This should panic due to invalid scheme
        .unwrap();
}

