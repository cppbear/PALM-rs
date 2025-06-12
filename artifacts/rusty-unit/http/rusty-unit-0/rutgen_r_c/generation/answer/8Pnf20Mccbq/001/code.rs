// Answer 0

#[test]
fn test_uri_valid() {
    // Create the necessary structs
    let uri = Uri {
        scheme: Scheme::Http,
        authority: Authority::from_static("example.com"),
        path_and_query: PathAndQuery::from_static("/"),
    };

    let parts = Parts {
        method: Method::GET,
        uri: uri.clone(),
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let request: Request<()> = Request::from_parts(parts, ());

    // Check the URI from the request
    assert_eq!(request.uri(), &uri);
}

#[test]
fn test_uri_empty() {
    // Create an empty Uri
    let uri = Uri {
        scheme: Scheme::Http,
        authority: Authority::from_static(""),
        path_and_query: PathAndQuery::from_static(""),
    };

    let parts = Parts {
        method: Method::GET,
        uri: uri.clone(),
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let request: Request<()> = Request::from_parts(parts, ());

    // Check that the URI is still accessible
    assert_eq!(request.uri(), &uri);
}

#[test]
#[should_panic]
fn test_uri_panics_without_parts() {
    // Create a request without the necessary parts (invalid state)
    let request: Request<()> = Request::new(()); // Implicit parts initialization

    // Trying to access the URI here
    // This should panic as we're not using valid parts
    let _ = request.uri();
}

