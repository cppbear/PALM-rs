// Answer 0


use std::fmt;

struct MockFormatter {
    buffer: String,
}

impl fmt::Write for MockFormatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

struct SampleStruct;

impl SampleStruct {
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string key")
    }
}

#[test]
fn test_expecting_valid() {
    let sample = SampleStruct;
    let mut mock_formatter = MockFormatter { buffer: String::new() };
    
    let result = sample.expecting(&mut mock_formatter);
    
    assert!(result.is_ok());
    assert_eq!(mock_formatter.buffer, "a string key");
}

#[test]
#[should_panic]
fn test_expecting_with_panic() {
    struct PanickingFormatter;
    
    impl fmt::Write for PanickingFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            panic!("Intentional panic");
        }
    }

    let sample = SampleStruct;
    let mut panicking_formatter = PanickingFormatter;
    
    sample.expecting(&mut panicking_formatter);
}


