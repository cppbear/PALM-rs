// Answer 0

#[test]
fn test_fmt_empty_data() {
    let data = ByteStr {
        bytes: Bytes::from_static(b""),
    };
    let path_and_query = PathAndQuery {
        data,
        query: NONE,
    };
    let mut formatter = fmt::Formatter::new();
    let _ = path_and_query.fmt(&mut formatter);
}

#[test]
fn test_fmt_data_start_with_slash() {
    let data = ByteStr {
        bytes: Bytes::from_static(b"/valid/path"),
    };
    let path_and_query = PathAndQuery {
        data,
        query: NONE,
    };
    let mut formatter = fmt::Formatter::new();
    let _ = path_and_query.fmt(&mut formatter);
}

#[test]
fn test_fmt_data_start_with_asterisk() {
    let data = ByteStr {
        bytes: Bytes::from_static(b"*valid/path"),
    };
    let path_and_query = PathAndQuery {
        data,
        query: NONE,
    };
    let mut formatter = fmt::Formatter::new();
    let _ = path_and_query.fmt(&mut formatter);
}

#[test]
fn test_fmt_data_start_with_other_character() {
    let data = ByteStr {
        bytes: Bytes::from_static(b"valid/path"),
    };
    let path_and_query = PathAndQuery {
        data,
        query: NONE,
    };
    let mut formatter = fmt::Formatter::new();
    let _ = path_and_query.fmt(&mut formatter);
}

