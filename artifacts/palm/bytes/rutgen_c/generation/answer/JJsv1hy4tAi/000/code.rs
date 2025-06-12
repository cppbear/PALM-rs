// Answer 0

#[test]
fn test_lower_hex_bytes_ref() {
    struct TestBytesRef<'a>(&'a [u8]);
    
    impl LowerHex for TestBytesRef<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for &b in self.0 {
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let data = TestBytesRef(&[10, 255, 0, 1]);
    let mut output = String::new();
    let result = write!(&mut output, "{:x}", data);
    
    assert!(result.is_ok());
    assert_eq!(output, "0aff00");
}

#[test]
fn test_lower_hex_bytes_ref_empty() {
    struct TestBytesRef<'a>(&'a [u8]);
    
    impl LowerHex for TestBytesRef<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for &b in self.0 {
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let data = TestBytesRef(&[]);
    let mut output = String::new();
    let result = write!(&mut output, "{:x}", data);
    
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_lower_hex_bytes_ref_single_byte() {
    struct TestBytesRef<'a>(&'a [u8]);
    
    impl LowerHex for TestBytesRef<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for &b in self.0 {
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let data = TestBytesRef(&[1]);
    let mut output = String::new();
    let result = write!(&mut output, "{:x}", data);
    
    assert!(result.is_ok());
    assert_eq!(output, "01");
}

