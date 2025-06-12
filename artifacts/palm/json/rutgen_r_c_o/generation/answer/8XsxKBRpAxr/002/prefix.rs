// Answer 0

#[test]
fn test_end_array_with_current_indent_zero_and_has_value() {
    struct MockWriter {
        success: bool,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.success {
                Ok(buf.len())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "write error"))
            }
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: true,
        indent: b"    ",
    };

    let mut writer = MockWriter { success: true };
    let result = formatter.end_array(&mut writer);
}

#[test]
#[should_panic]
fn test_end_array_with_current_indent_zero_and_has_value_and_indent_err() {
    struct MockWriter {
        success: bool,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.success {
                Ok(buf.len())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "write error"))
            }
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: true,
        indent: b"    ",
    };

    let mut writer = MockWriter { success: false };
    let result = formatter.end_array(&mut writer);
}

