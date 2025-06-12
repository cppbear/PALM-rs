// Answer 0

#[test]
fn test_extensions_ref_with_no_error() {
    struct CustomBuilder {
        inner: Result<Parts>,
    }

    let extensions = Extensions::default();
    let parts = Parts {
        extensions: extensions.clone(),
        ..Default::default()
    };

    let builder = CustomBuilder {
        inner: Ok(parts),
    };

    let result = builder.extensions_ref();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), &extensions);
}

#[test]
fn test_extensions_ref_with_error() {
    struct CustomBuilder {
        inner: Result<Parts>,
    }

    let builder = CustomBuilder {
        inner: Err(Error(/* initialization */)),
    };

    let result = builder.extensions_ref();
    assert!(result.is_none());
}

