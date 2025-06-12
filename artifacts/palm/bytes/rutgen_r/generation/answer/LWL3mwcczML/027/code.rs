// Answer 0

#[test]
fn test_fmt_empty() {
    struct Bytes(Vec<u8>);
    
    impl std::fmt::Debug for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in &self.0 {
                if b == b'\n' {
                    write!(f, "\\n")?;
                } else if b == b'\r' {
                    write!(f, "\\r")?;
                } else if b == b'\t' {
                    write!(f, "\\t")?;
                } else if b == b'\\' || b == b'"' {
                    write!(f, "\\{}", b as char)?;
                } else if b == b'\0' {
                    write!(f, "\\0")?;
                } else if (0x20..0x7f).contains(&b) {
                    write!(f, "{}", b as char)?;
                } else {
                    write!(f, "\\x{:02x}", b)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let bytes = Bytes(vec![]);
    let result = format!("{:?}", bytes);
    assert_eq!(result, "b\"\"");
}

#[test]
fn test_fmt_with_printable() {
    struct Bytes(Vec<u8>);
    
    impl std::fmt::Debug for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in &self.0 {
                if b == b'\n' {
                    write!(f, "\\n")?;
                } else if b == b'\r' {
                    write!(f, "\\r")?;
                } else if b == b'\t' {
                    write!(f, "\\t")?;
                } else if b == b'\\' || b == b'"' {
                    write!(f, "\\{}", b as char)?;
                } else if b == b'\0' {
                    write!(f, "\\0")?;
                } else if (0x20..0x7f).contains(&b) {
                    write!(f, "{}", b as char)?;
                } else {
                    write!(f, "\\x{:02x}", b)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let bytes = Bytes(vec![b'a', b'b', b'c']);
    let result = format!("{:?}", bytes);
    assert_eq!(result, "b\"abc\"");
}

#[test]
fn test_fmt_with_special_chars() {
    struct Bytes(Vec<u8>);
    
    impl std::fmt::Debug for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in &self.0 {
                if b == b'\n' {
                    write!(f, "\\n")?;
                } else if b == b'\r' {
                    write!(f, "\\r")?;
                } else if b == b'\t' {
                    write!(f, "\\t")?;
                } else if b == b'\\' || b == b'"' {
                    write!(f, "\\{}", b as char)?;
                } else if b == b'\0' {
                    write!(f, "\\0")?;
                } else if (0x20..0x7f).contains(&b) {
                    write!(f, "{}", b as char)?;
                } else {
                    write!(f, "\\x{:02x}", b)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let bytes = Bytes(vec![b'\n', b'\r', b'\t', b'\\', b'"', b'\0', b'k']);
    let result = format!("{:?}", bytes);
    assert_eq!(result, "b\"\\n\\r\\tk\\0\\\"\\\\\"");
}

#[test]
fn test_fmt_with_non_printable() {
    struct Bytes(Vec<u8>);
    
    impl std::fmt::Debug for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in &self.0 {
                if b == b'\n' {
                    write!(f, "\\n")?;
                } else if b == b'\r' {
                    write!(f, "\\r")?;
                } else if b == b'\t' {
                    write!(f, "\\t")?;
                } else if b == b'\\' || b == b'"' {
                    write!(f, "\\{}", b as char)?;
                } else if b == b'\0' {
                    write!(f, "\\0")?;
                } else if (0x20..0x7f).contains(&b) {
                    write!(f, "{}", b as char)?;
                } else {
                    write!(f, "\\x{:02x}", b)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let bytes = Bytes(vec![b'\x01', b'\x02', b'\x03']);
    let result = format!("{:?}", bytes);
    assert_eq!(result, "b\"\\x01\\x02\\x03\"");
}

