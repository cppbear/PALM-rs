// Answer 0

fn test_fmt_with_newline() {
    struct ByteWrapper(&'static [u8]);

    impl std::fmt::Debug for ByteWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let result = write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\n' {
                    write!(f, "\\n")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(result)
        }
    }

    let data = ByteWrapper(&[b'\n']);
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\n""#);
}

fn test_fmt_with_carriage_return() {
    struct ByteWrapper(&'static [u8]);

    impl std::fmt::Debug for ByteWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let result = write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\r' {
                    write!(f, "\\r")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(result)
        }
    }

    let data = ByteWrapper(&[b'\r']);
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\r""#);
}

fn test_fmt_with_tab() {
    struct ByteWrapper(&'static [u8]);

    impl std::fmt::Debug for ByteWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let result = write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\t' {
                    write!(f, "\\t")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(result)
        }
    }

    let data = ByteWrapper(&[b'\t']);
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\t""#);
}

fn test_fmt_with_escape() {
    struct ByteWrapper(&'static [u8]);

    impl std::fmt::Debug for ByteWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let result = write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\\' {
                    write!(f, "\\{}", b as char)?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(result)
        }
    }

    let data = ByteWrapper(&[b'\\']);
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\\""#);
}

fn test_fmt_with_non_printable() {
    struct ByteWrapper(&'static [u8]);

    impl std::fmt::Debug for ByteWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let result = write!(f, "b\"")?;
            for &b in self.0 {
                if !b.is_ascii_graphic() && b != b'\n' && b != b'\r' && b != b'\t' && b != b'\\' {
                    write!(f, "\\x{:02x}", b)?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(result)
        }
    }

    let data = ByteWrapper(&[b'\x01']);
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b"\x01""#);
}

fn test_fmt_with_empty() {
    struct ByteWrapper(&'static [u8]);

    impl std::fmt::Debug for ByteWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let result = write!(f, "b\"")?;
            write!(f, "\"")?;
            Ok(result)
        }
    }

    let data = ByteWrapper(&[]);
    let mut output = vec![];
    let result = std::fmt::write(&mut output, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), r#"b""""#);
}

