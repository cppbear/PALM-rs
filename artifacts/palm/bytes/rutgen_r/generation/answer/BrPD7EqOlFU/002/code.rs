// Answer 0

#[derive(Debug)]
struct HexFormatter<'a>(&'a [u8]);

impl<'a> std::fmt::Display for HexFormatter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in self.0 {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}

#[test]
fn test_hex_formatter_with_empty_slice() {
    let hex_formatter = HexFormatter(&[]);
    let result = format!("{}", hex_formatter);
    assert_eq!(result, "");
}

#[test]
fn test_hex_formatter_with_one_byte() {
    let hex_formatter = HexFormatter(&[0x0A]);
    let result = format!("{}", hex_formatter);
    assert_eq!(result, "0A");
}

#[test]
fn test_hex_formatter_with_multiple_bytes() {
    let hex_formatter = HexFormatter(&[0x01, 0xFF, 0xAB, 0xCD]);
    let result = format!("{}", hex_formatter);
    assert_eq!(result, "01FFABCD");
}

#[test]
fn test_hex_formatter_with_boundary_values() {
    let hex_formatter = HexFormatter(&[0x00, 0x7F, 0x80, 0xFF]);
    let result = format!("{}", hex_formatter);
    assert_eq!(result, "007F80FF");
}

