// Answer 0

#[test]
fn test_fmt_with_slash() {
    let bytes = Bytes::from_static(b"/path");
    let data = ByteStr { bytes };
    let path_and_query = PathAndQuery { data, query: NONE };

    let mut output = String::new();
    {
        let fmt = &mut output as &mut dyn fmt::Formatter;
        let _ = path_and_query.fmt(fmt);
    }
    
    assert_eq!(output, "/path");
}

#[test]
fn test_fmt_with_asterisk() {
    let bytes = Bytes::from_static(b"*path");
    let data = ByteStr { bytes };
    let path_and_query = PathAndQuery { data, query: NONE };

    let mut output = String::new();
    {
        let fmt = &mut output as &mut dyn fmt::Formatter;
        let _ = path_and_query.fmt(fmt);
    }
    
    assert_eq!(output, "*path");
}

#[test]
fn test_fmt_with_non_empty() {
    let bytes = Bytes::from_static(b"path");
    let data = ByteStr { bytes };
    let path_and_query = PathAndQuery { data, query: NONE };

    let mut output = String::new();
    {
        let fmt = &mut output as &mut dyn fmt::Formatter;
        let _ = path_and_query.fmt(fmt);
    }

    assert_eq!(output, "/path");
}

#[test]
fn test_fmt_with_empty() {
    let bytes = Bytes::from_static(b"");
    let data = ByteStr { bytes };
    let path_and_query = PathAndQuery { data, query: NONE };

    let mut output = String::new();
    {
        let fmt = &mut output as &mut dyn fmt::Formatter;
        let _ = path_and_query.fmt(fmt);
    }

    assert_eq!(output, "/");
}

