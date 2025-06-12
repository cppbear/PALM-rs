// Answer 0

#[test]
fn test_fmt_not_empty_starts_with_slash() {
    let data = ByteStr {
        bytes: Bytes::from_static(b"/example"),
    };
    let path_and_query = PathAndQuery {
        data,
        query: NONE,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", path_and_query);
    assert!(result.is_ok());
    assert_eq!(output, "/example");
}

#[test]
fn test_fmt_not_empty_starts_with_asterisk() {
    let data = ByteStr {
        bytes: Bytes::from_static(b"*example"),
    };
    let path_and_query = PathAndQuery {
        data,
        query: NONE,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", path_and_query);
    assert!(result.is_ok());
    assert_eq!(output, "*example");
}

#[test]
fn test_fmt_not_empty_neither_slash_nor_asterisk() {
    let data = ByteStr {
        bytes: Bytes::from_static(b"example"),
    };
    let path_and_query = PathAndQuery {
        data,
        query: NONE,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", path_and_query);
    assert!(result.is_ok());
    assert_eq!(output, "/example");
}

#[test]
fn test_fmt_empty_case() {
    let data = ByteStr {
        bytes: Bytes::from_static(b""),
    };
    let path_and_query = PathAndQuery {
        data,
        query: NONE,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", path_and_query);
    assert!(result.is_ok());
    assert_eq!(output, "/");
}

