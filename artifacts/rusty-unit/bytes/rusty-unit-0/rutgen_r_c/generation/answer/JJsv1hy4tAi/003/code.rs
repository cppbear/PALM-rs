// Answer 0

#[test]
fn test_lower_hex_bytes_ref_empty() {
    struct BytesRef<'a>(&'a [u8]);
    
    impl LowerHex for BytesRef<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for &b in self.0 {
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let bytes_ref = BytesRef(&[]);
    let mut formatter = Formatter::new();
    let result = bytes_ref.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_lower_hex_bytes_ref_single_byte() {
    struct BytesRef<'a>(&'a [u8]);
    
    impl LowerHex for BytesRef<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for &b in self.0 {
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let bytes_ref = BytesRef(&[0x01]);
    let mut formatter = Formatter::new();
    let result = bytes_ref.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_lower_hex_bytes_ref_multiple_bytes() {
    struct BytesRef<'a>(&'a [u8]);
    
    impl LowerHex for BytesRef<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for &b in self.0 {
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let bytes_ref = BytesRef(&[0x01, 0xAB, 0xFF]);
    let mut formatter = Formatter::new();
    let result = bytes_ref.fmt(&mut formatter);
    assert!(result.is_ok());
}

