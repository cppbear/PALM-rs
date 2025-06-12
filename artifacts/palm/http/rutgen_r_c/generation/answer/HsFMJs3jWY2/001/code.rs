// Answer 0

#[test]
fn test_parts_debug_fmt() {
    use crate::header::{HeaderMap, HeaderName, HeaderValue};
    use crate::method::Method;
    use crate::version::Version;
    use crate::{Extensions, Uri};
    
    let method = Method(/* initialization here */);
    let uri = Uri { 
        scheme: Scheme(/* initialization here */),
        authority: Authority(/* initialization here */),
        path_and_query: PathAndQuery(/* initialization here */),
    };
    let version = Version(/* initialization here */);
    
    let mut headers = HeaderMap::default();
    headers.insert(HeaderName::from_static("Content-Type"), HeaderValue::from_static("application/json"));
    
    let parts = Parts {
        method,
        uri,
        version,
        headers,
        extensions: Extensions::default(),
        _priv: (),
    };

    let output = format!("{:?}", parts);
    assert!(output.contains("Parts"));
    assert!(output.contains("method"));
    assert!(output.contains("uri"));
    assert!(output.contains("version"));
    assert!(output.contains("headers"));
}

#[test]
fn test_parts_debug_fmt_empty_headers() {
    use crate::header::{HeaderMap};
    use crate::method::Method;
    use crate::version::Version;
    use crate::{Extensions, Uri};

    let method = Method(/* initialization here */);
    let uri = Uri { 
        scheme: Scheme(/* initialization here */),
        authority: Authority(/* initialization here */),
        path_and_query: PathAndQuery(/* initialization here */),
    };
    let version = Version(/* initialization here */);
    
    let headers = HeaderMap::default();
    
    let parts = Parts {
        method,
        uri,
        version,
        headers,
        extensions: Extensions::default(),
        _priv: (),
    };

    let output = format!("{:?}", parts);
    assert!(output.contains("Parts"));
    assert!(output.contains("method"));
    assert!(output.contains("uri"));
    assert!(output.contains("version"));
    assert!(output.contains("headers"));
}

