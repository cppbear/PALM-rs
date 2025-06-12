// Answer 0

#[test]
fn test_end_object_no_value() {
    struct TestWriter(Vec<u8>);
    
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: false,
        indent: b"  ",
    };

    let result = formatter.end_object(&mut writer);
    assert!(result.is_ok());
    assert_eq!(writer.0, b"}");
}

#[test]
fn test_end_object_with_value_and_indent() {
    struct TestWriter(Vec<u8>);
    
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    let mut formatter = PrettyFormatter {
        current_indent: 3,
        has_value: true,
        indent: b"  ",
    };

    let result = formatter.end_object(&mut writer);
    assert!(result.is_ok());
    assert_eq!(writer.0, b"\n      }");
}

