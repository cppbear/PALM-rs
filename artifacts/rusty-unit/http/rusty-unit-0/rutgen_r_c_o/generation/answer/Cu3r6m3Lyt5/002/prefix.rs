// Answer 0

#[test]
fn test_fmt_with_slash_prefix() {
    let path_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from_static(b"/example"),
        },
        query: NONE,
    };
    let mut output = String::new();
    path_query.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_with_asterisk_prefix() {
    let path_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from_static(b"*example"),
        },
        query: NONE,
    };
    let mut output = String::new();
    path_query.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_with_slash_and_additional_bytes() {
    let path_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from_static(b"/test/path"),
        },
        query: NONE,
    };
    let mut output = String::new();
    path_query.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_with_asterisk_and_additional_bytes() {
    let path_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from_static(b"*test/path"),
        },
        query: NONE,
    };
    let mut output = String::new();
    path_query.fmt(&mut output).unwrap();
}

