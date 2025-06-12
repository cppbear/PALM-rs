// Answer 0

fn test_fmt_with_data_starting_with_slash() {
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

    let path = Path { data: String::from("/test/path") };
    let mut output = String::new();
    let result = path.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

fn test_fmt_with_data_starting_with_star() {
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

    let path = Path { data: String::from("*test/path") };
    let mut output = String::new();
    let result = path.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

fn test_fmt_with_non_empty_data_not_starting_with_slash_or_star() {
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

    let path = Path { data: String::from("test/path") };
    let mut output = String::new();
    let result = path.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

fn test_fmt_with_empty_data() {
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
    let result = path.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

