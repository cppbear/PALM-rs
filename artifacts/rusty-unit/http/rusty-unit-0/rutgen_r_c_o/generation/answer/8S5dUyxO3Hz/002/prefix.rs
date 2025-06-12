// Answer 0

#[test]
fn test_path_and_query_with_none_scheme_and_non_empty_authority() {
    let authority = Authority {
        data: ByteStr {
            bytes: Bytes::from_static(b"non_empty_authority"),
        },
    };
    let scheme = Scheme {
        inner: Scheme2::None,
    };
    let path_and_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from_static(b"/path/to/resource"),
        },
        query: 0,
    };
    let uri = Uri {
        scheme,
        authority,
        path_and_query,
    };
    let result = uri.path_and_query();
}

#[test]
fn test_path_and_query_with_none_scheme_and_non_empty_authority_alternate() {
    let authority = Authority {
        data: ByteStr {
            bytes: Bytes::from_static(b"another_authority"),
        },
    };
    let scheme = Scheme {
        inner: Scheme2::None,
    };
    let path_and_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from_static(b"/another/path"),
        },
        query: 1,
    };
    let uri = Uri {
        scheme,
        authority,
        path_and_query,
    };
    let result = uri.path_and_query();
}

