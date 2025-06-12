// Answer 0

#[derive(Debug)]
struct HexFormatter<'a>(&'a [u8]);

impl<'a> std::fmt::Format for HexFormatter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in self.0 {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}

#[test]
fn test_hex_formatter_empty() {
    let bytes = HexFormatter(&[]);
    let result = format!("{}", bytes);
    assert_eq!(result, "");
}

#[test]
fn test_hex_formatter_single_byte() {
    let bytes = HexFormatter(&[0x0A]);
    let result = format!("{}", bytes);
    assert_eq!(result, "0A");
}

#[test]
fn test_hex_formatter_multiple_bytes() {
    let bytes = HexFormatter(&[0x01, 0xFF, 0x4A]);
    let result = format!("{}", bytes);
    assert_eq!(result, "01FF4A");
}

#[test]
#[should_panic]
fn test_hex_formatter_panic_invalid_write() {
    struct InvalidFormatter;

    impl std::fmt::Write for InvalidFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Err(std::fmt::Error)
        }
    }

    let bytes = HexFormatter(&[0x01, 0x02]);
    let mut invalid_f = InvalidFormatter;
    let _ = bytes.fmt(&mut invalid_f);
}

