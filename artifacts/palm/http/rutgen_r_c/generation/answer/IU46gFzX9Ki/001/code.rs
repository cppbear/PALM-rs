// Answer 0

#[test]
fn test_scheme_str_none() {
    struct MockAuthority {
        data: ByteStr,
    }

    struct MockPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    let uri = Uri {
        scheme: Scheme::empty(),
        authority: MockAuthority { data: ByteStr { bytes: Bytes::from_static(b"authority") } },
        path_and_query: MockPathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"path") }, query: 0 },
    };

    assert_eq!(uri.scheme_str(), None);
}

