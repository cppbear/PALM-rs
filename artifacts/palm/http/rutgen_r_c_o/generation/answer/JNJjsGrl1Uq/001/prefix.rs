// Answer 0

#[test]
fn test_version_http_11() {
    let version = Version(Http(1_1));
    let headers = HeaderMap::new();
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: version.clone(),
        headers,
        extensions: Extensions::new(),
        _priv: (),
    };
    let response: Response<()> = Response::from_parts(parts, ());
    let _result = response.version();
}

#[test]
fn test_version_http_10() {
    let version = Version(Http(1_0));
    let headers = HeaderMap::new();
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: version.clone(),
        headers,
        extensions: Extensions::new(),
        _priv: (),
    };
    let response: Response<()> = Response::from_parts(parts, ());
    let _result = response.version();
}

#[test]
fn test_version_http_2() {
    let version = Version(Http(2));
    let headers = HeaderMap::new();
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: version.clone(),
        headers,
        extensions: Extensions::new(),
        _priv: (),
    };
    let response: Response<()> = Response::from_parts(parts, ());
    let _result = response.version();
}

#[test]
fn test_version_http_3() {
    let version = Version(Http(3));
    let headers = HeaderMap::new();
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: version.clone(),
        headers,
        extensions: Extensions::new(),
        _priv: (),
    };
    let response: Response<()> = Response::from_parts(parts, ());
    let _result = response.version();
}

#[test]
fn test_version_http_09() {
    let version = Version(Http(0_9));
    let headers = HeaderMap::new();
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: version.clone(),
        headers,
        extensions: Extensions::new(),
        _priv: (),
    };
    let response: Response<()> = Response::from_parts(parts, ());
    let _result = response.version();
}

