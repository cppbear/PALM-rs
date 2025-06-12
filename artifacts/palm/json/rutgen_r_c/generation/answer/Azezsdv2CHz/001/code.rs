// Answer 0

#[test]
fn test_end_object_writer_error() {
    use std::io::{self, Write};

    struct MockWriter {
        should_fail: bool,
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail {
                Err(io::Error::new(io::ErrorKind::Other, "Write error"))
            } else {
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let indent = b"  ";
    let mut formatter = PrettyFormatter {
        current_indent: 1,
        has_value: true,
        indent,
    };
    let mut writer = MockWriter { should_fail: true };

    let result = formatter.end_object(&mut writer);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Write error");
}

