// Answer 0

#[test]
fn test_builder_creation() {
    use crate::Uri;

    let builder = Uri::builder();
    assert!(builder.parts.is_ok());
}

#[test]
fn test_builder_build_without_parts() {
    use crate::Uri;

    let builder = Uri::builder();
    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_builder_scheme_authority_and_path() {
    use crate::{Uri, Authority, PathAndQuery};

    struct ValidScheme;
    impl TryInto<Scheme> for ValidScheme {
        type Error = crate::Error;
        fn try_into(self) -> Result<Scheme, Self::Error> {
            Ok(Scheme { inner: Scheme2::new("https") })
        }
    }

    struct ValidAuthority;
    impl TryInto<Authority> for ValidAuthority {
        type Error = crate::Error;
        fn try_into(self) -> Result<Authority, Self::Error> {
            Ok(Authority { data: ByteStr::from("hyper.rs") })
        }
    }

    struct ValidPathAndQuery;
    impl TryInto<PathAndQuery> for ValidPathAndQuery {
        type Error = crate::Error;
        fn try_into(self) -> Result<PathAndQuery, Self::Error> {
            let data = ByteStr::from("/");
            Ok(PathAndQuery { data, query: 0 })
        }
    }

    let builder = Uri::builder()
        .scheme(ValidScheme {})
        .authority(ValidAuthority {})
        .path_and_query(ValidPathAndQuery {});

    let uri_result = builder.build();
    assert!(uri_result.is_ok());
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_builder_invalid_static_str() {
    use crate::Uri;

    let invalid_uri = Uri::from_static("invalid_uri_with_space ");
}

