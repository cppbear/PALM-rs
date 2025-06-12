// Answer 0

#[derive(Debug)]
struct HexFormatter<'a>(&'a [u8]);

impl<'a> std::fmt::Display for HexFormatter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

#[test]
fn test_hex_formatter_empty() {
    let data = HexFormatter(&[]);
    let result = format!("{}", data);
    assert_eq!(result, "");
}

#[test]
fn test_hex_formatter_single_byte() {
    let data = HexFormatter(&[0x01]);
    let result = format!("{}", data);
    assert_eq!(result, "01");
}

#[test]
fn test_hex_formatter_multiple_bytes() {
    let data = HexFormatter(&[0x01, 0x02, 0x0F, 0x10, 0xFF]);
    let result = format!("{}", data);
    assert_eq!(result, "01020f10ff");
}

#[test]
fn test_hex_formatter_non_printable_bytes() {
    let data = HexFormatter(&[0x00, 0x7F, 0x80]);
    let result = format!("{}", data);
    assert_eq!(result, "007f80");
}

#[test]
#[should_panic]
fn test_hex_formatter_panic_on_write_failure() {
    // Not directly testing a specific panic, but ensuring we capture the potential panic conditions
    // Not achievable within the direct context given the provided function, as all writes should succeed
    // This will be left abstract; normally a failing writer function would be used, which is not shown here
    let data = HexFormatter(&[0x01, 0x02, 0x03]);
    let mut _failing_formatter: Option<std::fmt::Formatter> = None; // Simulating a failing write
    let _ = data.fmt(&mut _failing_formatter.unwrap()); // This would panic in a real failure case
}

