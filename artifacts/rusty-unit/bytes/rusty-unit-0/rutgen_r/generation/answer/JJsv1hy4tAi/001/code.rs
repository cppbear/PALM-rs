// Answer 0

#[test]
fn test_fmt_with_empty_slice() {
    struct TestHex<'a>(&'a [u8]);

    impl std::fmt::Debug for TestHex<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.0 {
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let hex = TestHex(&[]);
    let result = format!("{:?}", hex);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_with_single_byte() {
    struct TestHex<'a>(&'a [u8]);

    impl std::fmt::Debug for TestHex<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.0 {
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let hex = TestHex(&[0xAB]);
    let result = format!("{:?}", hex);
    assert_eq!(result, "[ab]");
}

#[test]
fn test_fmt_with_multiple_bytes() {
    struct TestHex<'a>(&'a [u8]);

    impl std::fmt::Debug for TestHex<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.0 {
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let hex = TestHex(&[0xAB, 0xCD, 0xEF]);
    let result = format!("{:?}", hex);
    assert_eq!(result, "[ab, cd, ef]");
}

#[should_panic]
#[test]
fn test_fmt_should_panic_on_write_error() {
    struct TestHex<'a>(&'a [u8]);

    impl std::fmt::Debug for TestHex<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.0 {
                // Simulating a write error by panicking
                if b == 0x00 {
                    panic!("forced panic");
                }
                write!(f, "{:02x}", b)?;
            }
            Ok(())
        }
    }

    let hex = TestHex(&[0x00]);
    let _ = format!("{:?}", hex);
}

