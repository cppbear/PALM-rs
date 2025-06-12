// Answer 0

#[test]
fn test_fmt_lower_hex_empty() {
    struct TestFormatter;
    
    impl Formatter<'_> {
        fn new() -> Self {
            TestFormatter {}
        }
    }

    let data = BytesRef(&[]);
    let mut formatter = TestFormatter::new();
    
    let result = data.fmt(&mut formatter);
    
    assert!(result.is_ok());
}

#[test]
fn test_fmt_lower_hex_single_byte() {
    struct TestFormatter {
        output: String,
    }
    
    impl TestFormatter {
        fn new() -> Self {
            TestFormatter { output: String::new() }
        }
        
        fn write(&mut self, s: &str) -> Result<(), std::fmt::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    let data = BytesRef(&[0x01]);
    let mut formatter = TestFormatter::new();

    let result = data.fmt(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter.output, "01");
}

#[test]
fn test_fmt_lower_hex_multiple_bytes() {
    struct TestFormatter {
        output: String,
    }
    
    impl TestFormatter {
        fn new() -> Self {
            TestFormatter { output: String::new() }
        }
        
        fn write(&mut self, s: &str) -> Result<(), std::fmt::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    let data = BytesRef(&[0x0A, 0xFF, 0x1C]);
    let mut formatter = TestFormatter::new();
    
    let result = data.fmt(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter.output, "0aff1c");
}

#[test]
#[should_panic]
fn test_fmt_lower_hex_panic_on_write_error() {
    struct TestFormatter;
    
    impl core::fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> Result {
            Err(core::fmt::Error) // Simulate write error
        }
    }

    let data = BytesRef(&[0x01, 0x02]);
    let mut formatter = TestFormatter;

    let _ = data.fmt(&mut formatter);
}

