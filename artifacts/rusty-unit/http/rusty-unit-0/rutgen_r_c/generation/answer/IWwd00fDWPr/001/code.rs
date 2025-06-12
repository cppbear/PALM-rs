// Answer 0

#[test]
fn test_scheme_with_valid_scheme() {
    struct ValidScheme;
    
    impl TryInto<Scheme> for ValidScheme {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            Ok(Scheme::Https)
        }
    }

    let builder = Builder::new();
    let updated_builder = builder.scheme(ValidScheme);
    
    assert!(updated_builder.parts.is_ok());
    assert!(updated_builder.parts.as_ref().unwrap().scheme.is_some());
}

#[test]
fn test_scheme_with_invalid_scheme() {
    struct InvalidScheme;
    
    impl TryInto<Scheme> for InvalidScheme {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            Err(crate::Error { inner: ErrorKind::InvalidScheme })
        }
    }

    let builder = Builder::new();
    let updated_builder = builder.scheme(InvalidScheme);

    assert!(updated_builder.parts.is_err());
}

#[test]
fn test_scheme_with_edge_case() {
    struct EdgeCaseScheme;

    impl TryInto<Scheme> for EdgeCaseScheme {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            Ok(Scheme::Http) // assuming HTTP is a valid edge case
        }
    }

    let builder = Builder::new();
    let updated_builder = builder.scheme(EdgeCaseScheme);

    assert!(updated_builder.parts.is_ok());
    assert!(updated_builder.parts.as_ref().unwrap().scheme.is_some());
}

