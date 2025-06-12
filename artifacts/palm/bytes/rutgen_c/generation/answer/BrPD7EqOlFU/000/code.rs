// Answer 0

#[test]
fn test_bytes_ref_upper_hex_empty() {
    struct BytesRef<'a>(&'a [u8]);
    impl core::fmt::UpperHex for BytesRef<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            for &b in self.0 {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let bytes_ref = BytesRef(&[]);
    let result = format!("{:X}", bytes_ref);
    assert_eq!(result, "");
}

#[test]
fn test_bytes_ref_upper_hex_single_byte() {
    struct BytesRef<'a>(&'a [u8]);
    impl core::fmt::UpperHex for BytesRef<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            for &b in self.0 {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let bytes_ref = BytesRef(&[0xAB]);
    let result = format!("{:X}", bytes_ref);
    assert_eq!(result, "AB");
}

#[test]
fn test_bytes_ref_upper_hex_multiple_bytes() {
    struct BytesRef<'a>(&'a [u8]);
    impl core::fmt::UpperHex for BytesRef<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            for &b in self.0 {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let bytes_ref = BytesRef(&[0xAB, 0xCD, 0xEF]);
    let result = format!("{:X}", bytes_ref);
    assert_eq!(result, "ABCDFF");
}

#[test]
fn test_bytes_ref_upper_hex_boundary_values() {
    struct BytesRef<'a>(&'a [u8]);
    impl core::fmt::UpperHex for BytesRef<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            for &b in self.0 {
                write!(f, "{:02X}", b)?;
            }
            Ok(())
        }
    }

    let bytes_ref = BytesRef(&[0x00, 0xFF]);
    let result = format!("{:X}", bytes_ref);
    assert_eq!(result, "00FF");
}

