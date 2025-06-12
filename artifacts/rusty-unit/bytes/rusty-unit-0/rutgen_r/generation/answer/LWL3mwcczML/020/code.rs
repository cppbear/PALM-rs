// Answer 0

#[test]
fn test_fmt_with_newline() {
    struct DebugByte<'a>(&'a [u8]);

    impl std::fmt::Debug for DebugByte<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
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
    
    let debug_bytes = DebugByte(&[b'a', b'\n', b'b']);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", debug_bytes);
    assert!(result.is_ok());
    assert_eq!(output, "b\"ab\\n\"");
}

#[test]
fn test_fmt_with_carriage_return() {
    struct DebugByte<'a>(&'a [u8]);

    impl std::fmt::Debug for DebugByte<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
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

    let debug_bytes = DebugByte(&[b'a', b'\r', b'b']);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", debug_bytes);
    assert!(result.is_ok());
    assert_eq!(output, "b\"ab\\r\"");
}

#[test]
fn test_fmt_with_tab() {
    struct DebugByte<'a>(&'a [u8]);

    impl std::fmt::Debug for DebugByte<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
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

    let debug_bytes = DebugByte(&[b'a', b'\t', b'b']);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", debug_bytes);
    assert!(result.is_ok());
    assert_eq!(output, "b\"a\\tb\"");
}

#[test]
fn test_fmt_with_backslash() {
    struct DebugByte<'a>(&'a [u8]);

    impl std::fmt::Debug for DebugByte<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
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

    let debug_bytes = DebugByte(&[b'a', b'\\', b'b']);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", debug_bytes);
    assert!(result.is_ok());
    assert_eq!(output, "b\"a\\\\b\"");
}

#[test]
fn test_fmt_with_quote() {
    struct DebugByte<'a>(&'a [u8]);

    impl std::fmt::Debug for DebugByte<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
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

    let debug_bytes = DebugByte(&[b'a', b'"', b'b']);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", debug_bytes);
    assert!(result.is_ok());
    assert_eq!(output, "b\"a\\\"b\"");
}

#[test]
fn test_fmt_with_null_byte() {
    struct DebugByte<'a>(&'a [u8]);

    impl std::fmt::Debug for DebugByte<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
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

    let debug_bytes = DebugByte(&[b'a', b'\0', b'b']);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", debug_bytes);
    assert!(result.is_ok());
    assert_eq!(output, "b\"a\\0b\"");
}

#[test]
fn test_fmt_with_printable_character() {
    struct DebugByte<'a>(&'a [u8]);

    impl std::fmt::Debug for DebugByte<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
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

    let debug_bytes = DebugByte(&[b'a', b'b', b'c']);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", debug_bytes);
    assert!(result.is_ok());
    assert_eq!(output, "b\"abc\"");
}

