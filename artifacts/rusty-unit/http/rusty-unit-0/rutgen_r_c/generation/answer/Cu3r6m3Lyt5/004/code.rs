// Answer 0

#[test]
fn test_fmt_with_slash_prefix() {
    use bytes::Bytes;
    use std::fmt;

    struct TestPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    impl fmt::Display for TestPathAndQuery {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            if !self.data.is_empty() {
                match self.data.as_bytes()[0] {
                    b'/' | b'*' => write!(fmt, "{}", &self.data[..]),
                    _ => write!(fmt, "/{}", &self.data[..]),
                }
            } else {
                write!(fmt, "/")
            }
        }
    }

    let valid_path = ByteStr {
        bytes: Bytes::from("/test/path"),
    };
    
    let path_query = TestPathAndQuery {
        data: valid_path,
        query: 0,
    };

    assert_eq!(format!("{}", path_query), "/test/path");
}

#[test]
fn test_fmt_with_asterisk_prefix() {
    use bytes::Bytes;
    use std::fmt;

    struct TestPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    impl fmt::Display for TestPathAndQuery {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            if !self.data.is_empty() {
                match self.data.as_bytes()[0] {
                    b'/' | b'*' => write!(fmt, "{}", &self.data[..]),
                    _ => write!(fmt, "/{}", &self.data[..]),
                }
            } else {
                write!(fmt, "/")
            }
        }
    }

    let valid_path = ByteStr {
        bytes: Bytes::from("*test/path"),
    };

    let path_query = TestPathAndQuery {
        data: valid_path,
        query: 0,
    };

    assert_eq!(format!("{}", path_query), "*test/path");
}

#[test]
fn test_fmt_with_non_slash_non_asterisk_prefix() {
    use bytes::Bytes;
    use std::fmt;

    struct TestPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    impl fmt::Display for TestPathAndQuery {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            if !self.data.is_empty() {
                match self.data.as_bytes()[0] {
                    b'/' | b'*' => write!(fmt, "{}", &self.data[..]),
                    _ => write!(fmt, "/{}", &self.data[..]),
                }
            } else {
                write!(fmt, "/")
            }
        }
    }

    let valid_path = ByteStr {
        bytes: Bytes::from("test/path"),
    };

    let path_query = TestPathAndQuery {
        data: valid_path,
        query: 0,
    };

    assert_eq!(format!("{}", path_query), "/test/path");
}

#[test]
fn test_fmt_with_empty_data() {
    use bytes::Bytes;
    use std::fmt;

    struct TestPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    impl fmt::Display for TestPathAndQuery {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            if !self.data.is_empty() {
                match self.data.as_bytes()[0] {
                    b'/' | b'*' => write!(fmt, "{}", &self.data[..]),
                    _ => write!(fmt, "/{}", &self.data[..]),
                }
            } else {
                write!(fmt, "/")
            }
        }
    }

    let empty_path = ByteStr {
        bytes: Bytes::from_static(&[]),
    };

    let path_query = TestPathAndQuery {
        data: empty_path,
        query: 0,
    };

    assert_eq!(format!("{}", path_query), "/");
}

