// Answer 0

#[derive(Debug)]
struct ByteArrayWrapper(&'static [u8]);

impl std::fmt::Debug for ByteArrayWrapper {
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

#[test]
fn test_empty_byte_array() {
    let bytes = ByteArrayWrapper(&[]);
    let mut output = vec![];
    let _ = std::fmt::write(&mut output, format_args!("{:?}", bytes));
    assert_eq!(String::from_utf8_lossy(&output), "b\"\"");
}

#[test]
fn test_byte_array_with_newline() {
    let bytes = ByteArrayWrapper(&[b'\n']);
    let mut output = vec![];
    let _ = std::fmt::write(&mut output, format_args!("{:?}", bytes));
    assert_eq!(String::from_utf8_lossy(&output), "b\"\\n\"");
}

#[test]
fn test_byte_array_with_carriage_return() {
    let bytes = ByteArrayWrapper(&[b'\r']);
    let mut output = vec![];
    let _ = std::fmt::write(&mut output, format_args!("{:?}", bytes));
    assert_eq!(String::from_utf8_lossy(&output), "b\"\\r\"");
}

#[test]
fn test_byte_array_with_tab() {
    let bytes = ByteArrayWrapper(&[b'\t']);
    let mut output = vec![];
    let _ = std::fmt::write(&mut output, format_args!("{:?}", bytes));
    assert_eq!(String::from_utf8_lossy(&output), "b\"\\t\"");
}

#[test]
fn test_byte_array_with_backslash() {
    let bytes = ByteArrayWrapper(&[b'\\']);
    let mut output = vec![];
    let _ = std::fmt::write(&mut output, format_args!("{:?}", bytes));
    assert_eq!(String::from_utf8_lossy(&output), "b\"\\\\\"");
}

#[test]
fn test_byte_array_with_double_quote() {
    let bytes = ByteArrayWrapper(&[b'"']);
    let mut output = vec![];
    let _ = std::fmt::write(&mut output, format_args!("{:?}", bytes));
    assert_eq!(String::from_utf8_lossy(&output), "b\"\\\"\"");
}

#[test]
fn test_byte_array_with_null() {
    let bytes = ByteArrayWrapper(&[b'\0']);
    let mut output = vec![];
    let _ = std::fmt::write(&mut output, format_args!("{:?}", bytes));
    assert_eq!(String::from_utf8_lossy(&output), "b\"\\0\"");
}

#[test]
fn test_byte_array_with_non_printable() {
    let bytes = ByteArrayWrapper(&[0x01, 0x02, 0xff]);
    let mut output = vec![];
    let _ = std::fmt::write(&mut output, format_args!("{:?}", bytes));
    assert_eq!(String::from_utf8_lossy(&output), "b\"\\x01\\x02\\xff\"");
}

