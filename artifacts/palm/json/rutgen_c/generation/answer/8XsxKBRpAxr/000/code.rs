// Answer 0

#[test]
fn test_end_array_empty() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let mut writer = MockWriter { output: Vec::new() };
    let formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };
    
    formatter.end_array(&mut writer).unwrap();
    assert_eq!(writer.output, b"]");
}

#[test]
fn test_end_array_with_value() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 1,
        has_value: true,
        indent: b"  ",
    };
    
    formatter.end_array(&mut writer).unwrap();
    assert_eq!(writer.output, b"\n]");
}

#[test]
fn test_end_array_with_multiple_calls() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: true,
        indent: b"  ",
    };
    
    // First call
    formatter.end_array(&mut writer).unwrap();
    assert_eq!(writer.output, b"\n]");
    
    // Reset for a second call
    writer.output.clear();
    formatter.current_indent = 0; // Simulate being back at top level
    formatter.end_array(&mut writer).unwrap();
    assert_eq!(writer.output, b"]");
}

