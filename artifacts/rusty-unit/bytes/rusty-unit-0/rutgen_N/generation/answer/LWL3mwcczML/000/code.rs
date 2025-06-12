// Answer 0

#[derive(Debug)]
struct ByteString(Vec<u8>);

impl std::fmt::Debug for ByteString {
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
fn test_empty_byte_string() {
    let bs = ByteString(vec![]);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", bs);
    assert_eq!(output, "b\"\"");
}

#[test]
fn test_printable_byte_string() {
    let bs = ByteString(vec![b'H', b'e', b'l', b'l', b'o']);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", bs);
    assert_eq!(output, "b\"Hello\"");
}

#[test]
fn test_special_characters() {
    let bs = ByteString(vec![b'H', b'e', b'\n', b'W', b'o', b'r', b'l', b'd', b'!']);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", bs);
    assert_eq!(output, "b\"He\\nWorld!\"");
}

#[test]
fn test_escape_characters() {
    let bs = ByteString(vec![b'T', b'h', b'i', b's', b'\\', b'"', b'!', b'\r']);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", bs);
    assert_eq!(output, "b\"This\\\"!\\r\"");
}

#[test]
fn test_control_characters() {
    let bs = ByteString(vec![b'B', b'a', b's', b'e', b' ', b'\x01', b'\x02', b'\x03']);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", bs);
    assert_eq!(output, "b\"Base \\x01\\x02\\x03\"");
}

