// Answer 0

#[test]
fn test_uri() {
    #[derive(Clone)]
    struct DummyBody;

    #[derive(Debug, Clone)]
    struct DummyMethod;

    #[derive(Debug, Clone)]
    struct DummyScheme;

    #[derive(Debug, Clone)]
    struct DummyAuthority;

    #[derive(Debug, Clone)]
    struct DummyPathAndQuery;

    let uri = Uri {
        scheme: DummyScheme,
        authority: DummyAuthority,
        path_and_query: DummyPathAndQuery,
    };

    let parts = Parts {
        method: DummyMethod,
        uri,
        version: Version::default(),
        headers: HeaderMap::default(),
        extensions: Extensions::default(),
        _priv: (),
    };

    let request: Request<DummyBody> = Request::from_parts(parts, DummyBody);

    assert_eq!(request.uri(), &request.head.uri);
}

