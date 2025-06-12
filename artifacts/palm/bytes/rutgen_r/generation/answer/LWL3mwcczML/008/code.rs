// Answer 0

fn fmt_debug_test() {
    use std::fmt::{self, Debug, Formatter};

    struct TestBytes<'a>(&'a [u8]);

    impl Debug for TestBytes<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "b\"")?;
            for &b in self.0 {
                if b == b'\n' {
                    write!(f, "\\n")?;
                } else if b == b'\r' {
                    write!(f, "\\r")?;
                } else if b == b'\t' {
                    write!(f, "\\t")?; // This should trigger an error as per the constraints
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

    // Test with a slice that includes valid byte values and specifically
    // includes b'\n' and b'\r', while also including b'\t' to test for the expected error.
    let bytes = TestBytes(&[b'\n', b'\r', b'\t']);
    
    // This should execute without panicking, but the write!(f, "\\t")? line should return an error.
    let result = fmt::write(&mut std::fmt::Formatter::new(), |f| bytes.fmt(f));

    assert_eq!(result.is_err(), true);
}

fn main() {
    fmt_debug_test();
}

