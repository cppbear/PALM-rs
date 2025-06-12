// Answer 0

#[test]
fn test_fmt_empty() {
    use std::fmt::{self, Formatter};

    struct HexFormatter(Vec<u8>);

    impl HexFormatter {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for &b in self.0.iter() {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let hex_formatter = HexFormatter(vec![]);
    let mut output = String::new();
    let result = hex_formatter.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_fmt_single_byte() {
    use std::fmt::{self, Formatter};

    struct HexFormatter(Vec<u8>);

    impl HexFormatter {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for &b in self.0.iter() {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let hex_formatter = HexFormatter(vec![0x1A]);
    let mut output = String::new();
    let result = hex_formatter.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "1A");
}

#[test]
fn test_fmt_multiple_bytes() {
    use std::fmt::{self, Formatter};

    struct HexFormatter(Vec<u8>);

    impl HexFormatter {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for &b in self.0.iter() {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let hex_formatter = HexFormatter(vec![0x12, 0x34, 0x56]);
    let mut output = String::new();
    let result = hex_formatter.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "123456");
}

#[test]
fn test_fmt_boundary_value() {
    use std::fmt::{self, Formatter};

    struct HexFormatter(Vec<u8>);

    impl HexFormatter {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for &b in self.0.iter() {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let hex_formatter = HexFormatter(vec![0x00, 0xFF]);
    let mut output = String::new();
    let result = hex_formatter.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "00FF");
}

