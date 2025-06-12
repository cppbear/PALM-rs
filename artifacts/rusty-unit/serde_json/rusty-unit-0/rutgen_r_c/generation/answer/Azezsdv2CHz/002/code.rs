// Answer 0

#[test]
fn test_end_object_with_has_value() {
    struct MockWriter {
        output: Vec<u8>,
        should_fail: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail {
                return Err(io::Error::new(io::ErrorKind::Other, "Mock error"));
            }
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {
        output: Vec::new(),
        should_fail: false,
    };
    
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: true,
        indent: b"  ",
    };

    // Test successful case
    let result = formatter.end_object(&mut writer);
    assert!(result.is_ok());
    assert_eq!(writer.output, b"\n  }");

    // Test failure case (panicking on indent call)
    writer.should_fail = true;
    let result = formatter.end_object(&mut writer);
    assert!(result.is_err());
}

