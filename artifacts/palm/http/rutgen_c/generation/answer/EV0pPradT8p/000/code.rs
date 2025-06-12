// Answer 0

#[test]
fn test_extensions_mut_success() {
    struct TestBuilder {
        inner: Result<Parts>,
    }

    let mut builder = TestBuilder {
        inner: Ok(Parts {
            extensions: Extensions {
                map: Some(Box::new(AnyMap::new())),
            },
            ..Default::default()
        }),
    };

    let extensions = builder.extensions_mut();
    assert!(extensions.is_some());
}

#[test]
fn test_extensions_mut_failure() {
    struct TestBuilder {
        inner: Result<Parts>,
    }

    let mut builder = TestBuilder {
        inner: Err(Error::new(ErrorKind::Other)), // Simulate an error
    };

    let extensions = builder.extensions_mut();
    assert!(extensions.is_none());
}

