// Answer 0

#[test]
fn test_headers_mut_empty() {
    let mut response: Response<()> = Response::new(());
    let headers_mut = response.headers_mut();
}

#[test]
fn test_headers_mut_insert_single() {
    let mut response: Response<()> = Response::new(());
    let headers_mut = response.headers_mut();
    headers_mut.insert(HeaderName::from_static("Host"), HeaderValue { inner: Bytes::new(), is_sensitive: false });
}

#[test]
fn test_headers_mut_insert_multiple() {
    let mut response: Response<()> = Response::new(());
    let headers_mut = response.headers_mut();
    headers_mut.insert(HeaderName::from_static("Host"), HeaderValue { inner: Bytes::new(), is_sensitive: false });
    headers_mut.insert(HeaderName::from_static("Content-Type"), HeaderValue { inner: Bytes::new(), is_sensitive: true });
}

#[test]
fn test_headers_mut_with_non_default_parts() {
    let mut parts = Parts {
        method: Method::GET,
        status: StatusCode::OK,
        version: Version::HTTP_2,
        headers: HeaderMap::default(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let mut response: Response<()> = Response::from_parts(parts, ());
    let headers_mut = response.headers_mut();
    headers_mut.insert(HeaderName::from_static("X-Custom-Header"), HeaderValue { inner: Bytes::new(), is_sensitive: false });
}

#[test]
fn test_headers_mut_large_map() {
    let mut response: Response<()> = Response::new(());
    let headers_mut = response.headers_mut();
    for i in 0..100 {
        headers_mut.insert(HeaderName::from_static(&format!("Header-{}", i)), HeaderValue { inner: Bytes::new(), is_sensitive: i % 2 == 0 });
    }
}

