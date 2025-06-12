// Answer 0

#[test]
fn test_fmt_empty() {
    struct HexFormatter(Vec<u8>);

    impl std::fmt::Display for HexFormatter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.0.iter() {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let hex_formatter = HexFormatter(vec![]);
    let result = format!("{}", hex_formatter);
    assert_eq!(result, "");
}

#[test]
fn test_fmt_single_byte() {
    struct HexFormatter(Vec<u8>);

    impl std::fmt::Display for HexFormatter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.0.iter() {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let hex_formatter = HexFormatter(vec![0x0A]);
    let result = format!("{}", hex_formatter);
    assert_eq!(result, "0A");
}

#[test]
fn test_fmt_multiple_bytes() {
    struct HexFormatter(Vec<u8>);

    impl std::fmt::Display for HexFormatter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.0.iter() {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let hex_formatter = HexFormatter(vec![0x01, 0xAB, 0xFF]);
    let result = format!("{}", hex_formatter);
    assert_eq!(result, "01ABFF");
}

#[test]
fn test_fmt_boundary_values() {
    struct HexFormatter(Vec<u8>);

    impl std::fmt::Display for HexFormatter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.0.iter() {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let hex_formatter = HexFormatter(vec![0x00, 0xFF]);
    let result = format!("{}", hex_formatter);
    assert_eq!(result, "00FF");
}

