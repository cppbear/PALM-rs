// Answer 0

#[test]
fn test_uri_with_full_initialization() {
    let scheme = Some(Scheme::Http);
    let authority = Some(Authority::from_str("example.com").unwrap());
    let path_and_query = Some(PathAndQuery::from_str("/path?query=1").unwrap());
    
    let uri = Uri {
        scheme: scheme.clone().unwrap(),
        authority: authority.clone().unwrap(),
        path_and_query: path_and_query.clone().unwrap(),
    };
    
    let headers = HeaderMap::new();
    let extensions = Extensions::new();
    
    let parts = Parts {
        method: Method::GET,
        uri,
        version: Version::HTTP_11,
        headers,
        extensions,
        _priv: (),
    };
    
    let request = Request::from_parts(parts, ());
    let result = request.uri();
}

#[test]
fn test_uri_with_no_authority() {
    let scheme = Some(Scheme::Http);
    let path_and_query = Some(PathAndQuery::from_str("/path").unwrap());
    
    let uri = Uri {
        scheme: scheme.clone().unwrap(),
        authority: None,
        path_and_query: path_and_query.clone().unwrap(),
    };
    
    let headers = HeaderMap::new();
    let extensions = Extensions::new();
    
    let parts = Parts {
        method: Method::GET,
        uri,
        version: Version::HTTP_11,
        headers,
        extensions,
        _priv: (),
    };
    
    let request = Request::from_parts(parts, ());
    let result = request.uri();
}

#[test]
fn test_uri_with_empty_path_and_query() {
    let scheme = Some(Scheme::Http);
    let authority = Some(Authority::from_str("example.com").unwrap());
    
    let uri = Uri {
        scheme: scheme.clone().unwrap(),
        authority: authority.clone().unwrap(),
        path_and_query: PathAndQuery::from_str("").unwrap(),
    };
    
    let headers = HeaderMap::new();
    let extensions = Extensions::new();
    
    let parts = Parts {
        method: Method::GET,
        uri,
        version: Version::HTTP_11,
        headers,
        extensions,
        _priv: (),
    };
    
    let request = Request::from_parts(parts, ());
    let result = request.uri();
}

#[test]
fn test_uri_with_https_scheme() {
    let scheme = Some(Scheme::Https);
    let authority = Some(Authority::from_str("secure.example.com").unwrap());
    let path_and_query = Some(PathAndQuery::from_str("/secure/path").unwrap());
    
    let uri = Uri {
        scheme: scheme.clone().unwrap(),
        authority: authority.clone().unwrap(),
        path_and_query: path_and_query.clone().unwrap(),
    };
    
    let headers = HeaderMap::new();
    let extensions = Extensions::new();
    
    let parts = Parts {
        method: Method::GET,
        uri,
        version: Version::HTTP_11,
        headers,
        extensions,
        _priv: (),
    };
    
    let request = Request::from_parts(parts, ());
    let result = request.uri();
}

#[test]
fn test_uri_with_no_scheme() {
    let authority = Some(Authority::from_str("example.com").unwrap());
    let path_and_query = Some(PathAndQuery::from_str("/path").unwrap());
    
    let uri = Uri {
        scheme: None,
        authority: authority.clone().unwrap(),
        path_and_query: path_and_query.clone().unwrap(),
    };
    
    let headers = HeaderMap::new();
    let extensions = Extensions::new();
    
    let parts = Parts {
        method: Method::GET,
        uri,
        version: Version::HTTP_11,
        headers,
        extensions,
        _priv: (),
    };
    
    let request = Request::from_parts(parts, ());
    let result = request.uri();
}

