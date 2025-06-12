// Answer 0

#[test]
fn test_pretty_formatter_new() {
    struct DummyWriter;
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    let formatter = PrettyFormatter::new();
    let mut writer = DummyWriter;

    // Test that the formatter initializes correctly
    assert_eq!(formatter.current_indent, 0);
    assert_eq!(formatter.has_value, false);
    assert_eq!(formatter.indent, b"  ");
}

#[test]
fn test_pretty_formatter_with_indent() {
    struct DummyWriter;
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let formatter = PrettyFormatter::with_indent(b"\t");
    let mut writer = DummyWriter;

    // Test the formatter with a custom indent
    assert_eq!(formatter.current_indent, 0);
    assert_eq!(formatter.has_value, false);
    assert_eq!(formatter.indent, b"\t");
}

