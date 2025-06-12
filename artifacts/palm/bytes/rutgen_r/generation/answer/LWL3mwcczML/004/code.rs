// Answer 0

fn fmt_test() -> Result<(), std::fmt::Error> {
    struct DebugBytes<'a>(&'a [u8]);

    impl<'a> std::fmt::Debug for DebugBytes<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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

    let input = DebugBytes(&[b'a', b'b', b'c', b'!', b'\0', b'\x01', b'\x1f']);
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        input.fmt(&mut formatter)?;
    }
    
    assert_eq!(output, "b\"abc!\\0\\x01\\x1f\"");
    Ok(())
}

#[test]
fn test_fmt() {
    fmt_test().unwrap();
}

#[test]
#[should_panic]
fn test_panic_on_invalid_write() {
    struct PanicBytes;
    
    impl std::fmt::Debug for PanicBytes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            // Directly cause a panic for this test
            Err(std::fmt::Error)
        }
    }

    let input = PanicBytes;
    let mut output = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut output);
    input.fmt(&mut formatter).unwrap();
}

