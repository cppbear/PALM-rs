// Answer 0

fn test_fmt_with_control_characters() {
    struct BytesWrapper<'a>(&'a [u8]);

    use std::fmt::{Formatter, Result};
    
    impl<'a> std::fmt::Debug for BytesWrapper<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

    let test_input = BytesWrapper(&[b'\n', b'\r', b'\t', b'\\', b'"']);
    let debug_output = format!("{:?}", test_input);
    assert_eq!(debug_output, r#"b"\n\r\t\\\""#);
}

fn test_fmt_with_non_printable_characters() {
    struct BytesWrapper<'a>(&'a [u8]);

    use std::fmt::{Formatter, Result};
    
    impl<'a> std::fmt::Debug for BytesWrapper<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

    let test_input = BytesWrapper(&[b'\x01', b'\x02', b'\x03']);
    let debug_output = format!("{:?}", test_input);
    assert_eq!(debug_output, r#"b"\x01\x02\x03""#);
}

fn test_fmt_with_all_characters() {
    struct BytesWrapper<'a>(&'a [u8]);

    use std::fmt::{Formatter, Result};
    
    impl<'a> std::fmt::Debug for BytesWrapper<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

    let test_input = BytesWrapper(&[b'\n', b'\r', b'\t', b'\\', b'"', b'a', b'\0', b'\xFF']);
    let debug_output = format!("{:?}", test_input);
    assert_eq!(debug_output, r#"b"\n\r\t\\\"a\xff""#);
}

