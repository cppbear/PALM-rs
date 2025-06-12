// Answer 0

#[test]
fn test_uri_mut_valid_uri() {
    let method = Method::GET;
    let uri = Uri {
        scheme: Some(Scheme::HTTP),
        authority: Some(Authority::from_static("example.com")),
        path_and_query: Some(PathAndQuery::from_static("/path")),
        _priv: (),
    };
    let parts = Parts {
        method,
        uri,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    let body = ();
    let mut request = Request::from_parts(parts, body);
    request.uri_mut();
}

#[test]
fn test_uri_mut_empty_path_and_query() {
    let method = Method::POST;
    let uri = Uri {
        scheme: Some(Scheme::HTTPS),
        authority: Some(Authority::from_static("example.com")),
        path_and_query: Some(PathAndQuery::from_static("")),
        _priv: (),
    };
    let parts = Parts {
        method,
        uri,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    let body = ();
    let mut request = Request::from_parts(parts, body);
    request.uri_mut();
}

#[test]
fn test_uri_mut_with_specific_path() {
    let method = Method::PUT;
    let uri = Uri {
        scheme: Some(Scheme::HTTP),
        authority: Some(Authority::from_static("api.example.com")),
        path_and_query: Some(PathAndQuery::from_static("/resource")),
        _priv: (),
    };
    let parts = Parts {
        method,
        uri,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    let body = ();
    let mut request = Request::from_parts(parts, body);
    request.uri_mut();
}

#[test]
fn test_uri_mut_valid_scheme_authority() {
    let method = Method::PATCH;
    let uri = Uri {
        scheme: Some(Scheme::FTP),
        authority: Some(Authority::from_static("ftp.example.com")),
        path_and_query: Some(PathAndQuery::from_static("/data")),
        _priv: (),
    };
    let parts = Parts {
        method,
        uri,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    let body = ();
    let mut request = Request::from_parts(parts, body);
    request.uri_mut();
}

