// Answer 0

#[derive(Debug)]
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

#[test]
fn test_fmt_with_newline() {
    let bytes = Bytes(vec![b'H', b'e', b'l', b'l', b'o', b' ', b'\n', b'W', b'o', b'r', b'l', b'd']);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"Hello \\nWorld\"");
}

#[test]
fn test_fmt_with_non_printable() {
    let bytes = Bytes(vec![b'\n', b'\t', b'A', b'\x01']);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\n\\tA\\x01\"");
}

#[test]
fn test_fmt_with_escape_characters() {
    let bytes = Bytes(vec![b'H', b'e', b'l', b'c', b'\\', b'"', b'o', b'k', b'!']);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"Helc\\\"ok!\"");
}

#[test]
fn test_fmt_with_zero_byte() {
    let bytes = Bytes(vec![b'\0', b'H', b'e']);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"\\0He\"");
}

