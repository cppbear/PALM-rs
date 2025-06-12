// Answer 0

#[test]
fn test_path_with_non_empty_path() {
    // Initialize a ByteStr with valid bytes for "/hello/world"
    let path_data = Bytes::from_static(b"/hello/world");
    let path_and_query = PathAndQuery {
        data: ByteStr { bytes: path_data },
        query: NONE,
    };

    assert_eq!(path_and_query.path(), "/hello/world");
}

#[test]
fn test_path_with_empty_path() {
    // Initialize a ByteStr with valid bytes for ""
    let path_data = Bytes::from_static(b"");
    let path_and_query = PathAndQuery {
        data: ByteStr { bytes: path_data },
        query: NONE,
    };

    assert_eq!(path_and_query.path(), "/");
}

#[test]
fn test_path_with_query() {
    // Initialize a ByteStr with valid bytes for "/hello/world" and set a query
    let path_data = Bytes::from_static(b"/hello/world?key=value");
    let path_and_query = PathAndQuery {
        data: ByteStr { bytes: path_data },
        query: 12, // index of '?'
    };

    assert_eq!(path_and_query.path(), "/hello/world");
}

#[test]
fn test_path_with_fragment() {
    // Initialize a ByteStr with valid bytes for "/hello/world#fragment"
    let path_data = Bytes::from_static(b"/hello/world#fragment");
    let path_and_query = PathAndQuery {
        data: ByteStr { bytes: path_data },
        query: 12, // index of '?', but in this case we won't apply as query is not equal to NONE
    };

    assert_eq!(path_and_query.path(), "/hello/world");
}

