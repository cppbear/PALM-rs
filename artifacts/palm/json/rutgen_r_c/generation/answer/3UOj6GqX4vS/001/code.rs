// Answer 0

#[test]
fn test_end_array_value_sets_has_value() {
    struct MockWriter(Vec<u8>);

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"    ",
    };
    
    let mut writer = MockWriter(Vec::new());

    // Call the method that is being tested
    let result = formatter.end_array_value(&mut writer);

    assert!(result.is_ok());
    assert!(formatter.has_value);
}

#[test]
fn test_end_array_value_multiple_calls() {
    struct MockWriter(Vec<u8>);

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"    ",
    };
    
    let mut writer = MockWriter(Vec::new());
    
    // First call
    let result1 = formatter.end_array_value(&mut writer);
    // Second call
    let result2 = formatter.end_array_value(&mut writer);
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(formatter.has_value);
}

