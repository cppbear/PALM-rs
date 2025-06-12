// Answer 0

#[test]
fn test_fmt_empty_data() {
    struct TestStruct {
        data: String,
    }

    impl TestStruct {
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

    let item = TestStruct { data: String::new() };
    let mut result = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut result);
    
    item.fmt(&mut formatter).unwrap();
    
    assert_eq!(result, "/");
}

#[test]
fn test_fmt_non_empty_data_starts_with_slash() {
    struct TestStruct {
        data: String,
    }

    impl TestStruct {
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

    let item = TestStruct { data: String::from("/path") };
    let mut result = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut result);
    
    item.fmt(&mut formatter).unwrap();
    
    assert_eq!(result, "/path");
}

#[test]
fn test_fmt_non_empty_data_does_not_start_with_slash() {
    struct TestStruct {
        data: String,
    }

    impl TestStruct {
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

    let item = TestStruct { data: String::from("path") };
    let mut result = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut result);
    
    item.fmt(&mut formatter).unwrap();
    
    assert_eq!(result, "/path");
}

#[test]
fn test_fmt_non_empty_data_starts_with_asterisk() {
    struct TestStruct {
        data: String,
    }

    impl TestStruct {
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

    let item = TestStruct { data: String::from("*path") };
    let mut result = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut result);
    
    item.fmt(&mut formatter).unwrap();
    
    assert_eq!(result, "*path");
}

