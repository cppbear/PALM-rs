// Answer 0

#[test]
fn test_fmt_with_slash_prefix() {
    struct Path {
        data: String,
    }

    impl Path {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    let path = Path { data: String::from("/example") };
    let mut output = String::new();
    {
        let _ = std::fmt::write(&mut output, format_args!("{}", path));
    }
    assert_eq!(output, "/example");
}

#[test]
fn test_fmt_with_asterisk_prefix() {
    struct Path {
        data: String,
    }

    impl Path {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    let path = Path { data: String::from("*example") };
    let mut output = String::new();
    {
        let _ = std::fmt::write(&mut output, format_args!("{}", path));
    }
    assert_eq!(output, "*example");
}

#[test]
fn test_fmt_without_prefix() {
    struct Path {
        data: String,
    }

    impl Path {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    let path = Path { data: String::from("example") };
    let mut output = String::new();
    {
        let _ = std::fmt::write(&mut output, format_args!("{}", path));
    }
    assert_eq!(output, "/example");
}

#[test]
fn test_fmt_empty_path() {
    struct Path {
        data: String,
    }

    impl Path {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    let path = Path { data: String::new() };
    let mut output = String::new();
    {
        let _ = std::fmt::write(&mut output, format_args!("{}", path));
    }
    assert_eq!(output, "/");
}

