// Answer 0

fn test_method_ref_no_error() {
    struct DummyMethod(Inner);
    struct DummyUri;
    struct DummyVersion;
    struct DummyHeaderMap;
    struct DummyExtensions;

    let method = DummyMethod(Inner);
    let uri = DummyUri;
    let version = DummyVersion;
    let headers = DummyHeaderMap;
    let extensions = DummyExtensions;

    let parts = Parts {
        method: method.clone(),
        uri,
        version,
        headers,
        extensions,
        _priv: (),
    };

    let builder = Builder {
        inner: Ok(parts),
    };

    assert_eq!(builder.method_ref(), Some(&method));
}

fn test_method_ref_with_error() {
    struct DummyMethod(Inner);
    struct DummyError;

    let builder = Builder {
        inner: Err(DummyError),
    };

    assert_eq!(builder.method_ref(), None);
}

fn test_method_ref_default_method() {
    struct DummyMethod(Inner);
    struct DummyUri;
    struct DummyVersion;
    struct DummyHeaderMap;
    struct DummyExtensions;

    let method = DummyMethod(Inner);
    let uri = DummyUri;
    let version = DummyVersion;
    let headers = DummyHeaderMap;
    let extensions = DummyExtensions;

    let default_parts = Parts {
        method: method,
        uri,
        version,
        headers,
        extensions,
        _priv: (),
    };

    let builder = Builder {
        inner: Ok(default_parts),
    };

    assert_eq!(builder.method_ref(), Some(&builder.inner.as_ref().unwrap().method));
}

