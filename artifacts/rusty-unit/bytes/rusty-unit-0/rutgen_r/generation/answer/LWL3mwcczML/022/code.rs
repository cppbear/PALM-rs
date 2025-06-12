// Answer 0

#[test]
fn test_fmt_with_newline() {
    struct BytesWrapper<'a>(&'a [u8]);

    impl std::fmt::Debug for BytesWrapper<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\n' {
                    write!(f, "\\n")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let data = BytesWrapper(b"Hello\nWorld");
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", data);
    assert_eq!(output, "b\"Hello\\nWorld\"");
}

#[test]
fn test_fmt_with_carriage_return() {
    struct BytesWrapper<'a>(&'a [u8]);

    impl std::fmt::Debug for BytesWrapper<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\r' {
                    write!(f, "\\r")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let data = BytesWrapper(b"Hello\rWorld");
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", data);
    assert_eq!(output, "b\"Hello\\rWorld\"");
}

#[test]
fn test_fmt_with_tab() {
    struct BytesWrapper<'a>(&'a [u8]);

    impl std::fmt::Debug for BytesWrapper<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\t' {
                    write!(f, "\\t")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let data = BytesWrapper(b"Hello\tWorld");
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", data);
    assert_eq!(output, "b\"Hello\\tWorld\"");
}

#[test]
fn test_fmt_with_backslash() {
    struct BytesWrapper<'a>(&'a [u8]);

    impl std::fmt::Debug for BytesWrapper<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\\' {
                    write!(f, "\\\\")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let data = BytesWrapper(b"Hello\\World");
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", data);
    assert_eq!(output, "b\"Hello\\\\World\"");
}

#[test]
fn test_fmt_with_double_quote() {
    struct BytesWrapper<'a>(&'a [u8]);

    impl std::fmt::Debug for BytesWrapper<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'"' {
                    write!(f, "\\\"")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let data = BytesWrapper(b"Hello\"World");
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", data);
    assert_eq!(output, "b\"Hello\\\"World\"");
}

#[test]
fn test_fmt_with_null_byte() {
    struct BytesWrapper<'a>(&'a [u8]);

    impl std::fmt::Debug for BytesWrapper<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\0' {
                    write!(f, "\\0")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let data = BytesWrapper(b"Hello\0World");
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", data);
    assert_eq!(output, "b\"Hello\\0World\"");
}

#[test]
fn test_fmt_with_printable_ascii() {
    struct BytesWrapper<'a>(&'a [u8]);

    impl std::fmt::Debug for BytesWrapper<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
                if (0x20..0x7f).contains(&b) {
                    write!(f, "{}", b as char)?;
                } else {
                    write!(f, "\\x{:02x}", b)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let data = BytesWrapper(b"Hello World!");
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", data);
    assert_eq!(output, "b\"Hello World!\"");
}

