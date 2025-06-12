// Answer 0

#[test]
fn test_as_str_empty() {
    let data = Bytes::from_static(&[]);
    let path_and_query = PathAndQuery {
        data: ByteStr { bytes: data },
        query: NONE,
    };
    path_and_query.as_str();
}

#[test]
fn test_as_str_base_path() {
    let data = Bytes::from_static(b"/hello/world");
    let path_and_query = PathAndQuery {
        data: ByteStr { bytes: data },
        query: NONE,
    };
    path_and_query.as_str();
}

#[test]
fn test_as_str_with_query() {
    let data = Bytes::from_static(b"/example?key=value&foo=bar");
    let path_and_query = PathAndQuery {
        data: ByteStr { bytes: data },
        query: 9,
    };
    path_and_query.as_str();
}

#[test]
fn test_as_str_path_only() {
    let data = Bytes::from_static(b"/");
    let path_and_query = PathAndQuery {
        data: ByteStr { bytes: data },
        query: NONE,
    };
    path_and_query.as_str();
}

#[test]
fn test_as_str_invalid_query() {
    let data = Bytes::from_static(b"/path#fragment");
    let path_and_query = PathAndQuery {
        data: ByteStr { bytes: data },
        query: NONE,
    };
    path_and_query.as_str();
}

