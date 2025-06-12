// Answer 0

#[derive(Debug)]
struct HexFormatter(Vec<u8>);

impl HexFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in self.0.iter() {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

#[test]
fn test_fmt_empty() {
    let formatter = HexFormatter(vec![]);
    let mut buffer = String::new();
    let result = formatter.fmt(&mut std::fmt::Formatter::from(&mut buffer));
    assert_eq!(result, Ok(()));
    assert_eq!(buffer, "");
}

#[test]
fn test_fmt_single_byte() {
    let formatter = HexFormatter(vec![0x01]);
    let mut buffer = String::new();
    let result = formatter.fmt(&mut std::fmt::Formatter::from(&mut buffer));
    assert_eq!(result, Ok(()));
    assert_eq!(buffer, "01");
}

#[test]
fn test_fmt_multiple_bytes() {
    let formatter = HexFormatter(vec![0x01, 0x2A, 0xFF]);
    let mut buffer = String::new();
    let result = formatter.fmt(&mut std::fmt::Formatter::from(&mut buffer));
    assert_eq!(result, Ok(()));
    assert_eq!(buffer, "012aff");
}

#[test]
fn test_fmt_boundary_bytes() {
    let formatter = HexFormatter(vec![0x00, 0xFF]);
    let mut buffer = String::new();
    let result = formatter.fmt(&mut std::fmt::Formatter::from(&mut buffer));
    assert_eq!(result, Ok(()));
    assert_eq!(buffer, "00ff");
}

