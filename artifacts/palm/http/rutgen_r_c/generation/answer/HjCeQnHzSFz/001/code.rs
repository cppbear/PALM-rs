// Answer 0

#[test]
fn test_builder_build_empty_parts() {
    let builder = Builder {
        parts: Err(Error { inner: ErrorKind::AuthorityMissing }),
    };
    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_builder_build_parts_with_scheme_only() {
    let builder = Builder {
        parts: Ok(Parts {
            scheme: Some(Scheme { inner: Scheme2::Http }),
            authority: None,
            path_and_query: None,
            _priv: (),
        }),
    };
    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_builder_build_parts_with_authority_and_path_only() {
    let builder = Builder {
        parts: Ok(Parts {
            scheme: None,
            authority: Some(Authority::empty()),
            path_and_query: Some(PathAndQuery::empty()),
            _priv: (),
        }),
    };
    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_builder_build_incomplete_parts() {
    let builder = Builder {
        parts: Ok(Parts {
            scheme: None,
            authority: None,
            path_and_query: None,
            _priv: (),
        }),
    };
    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_builder_build_all_parts_complete() {
    let builder = Builder {
        parts: Ok(Parts {
            scheme: Some(Scheme { inner: Scheme2::Http }),
            authority: Some(Authority::empty()),
            path_and_query: Some(PathAndQuery::empty()),
            _priv: (),
        }),
    };
    let result = builder.build();
    assert!(result.is_ok());
}

