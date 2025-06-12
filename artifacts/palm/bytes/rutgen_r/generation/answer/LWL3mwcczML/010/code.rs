// Answer 0

fn fmt_test() {
    struct ByteFormatter(Vec<u8>);

    use std::fmt::{self, Formatter, Write};

    impl ByteFormatter {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

    let data = ByteFormatter(vec![b'\n', b'\r', b'!', b'\0', b'\x0F']);
    let mut output = String::new();
    assert_eq!(data.fmt(&mut output), Ok(()));
    assert_eq!(output, r#"b"\n\r!\\0\x0f""#);
}

#[test]
fn test_format_newline() {
    fmt_test();
}

#[test]
fn test_format_carriage_return() {
    fmt_test();
}

#[test]
fn test_format_non_printable() {
    fmt_test();
}

#[test]
fn test_format_multiple_cases() {
    fmt_test();
}

