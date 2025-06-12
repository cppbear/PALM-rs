// Answer 0

fn test_begin_object_key_first_true_error() {
    struct TestWriter {
        should_error: bool,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
            } else {
                Ok(buf.len())
            }
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut test_writer = TestWriter { should_error: true };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ", // Example indent
    };

    let result = formatter.begin_object_key(&mut test_writer, true);
    assert!(result.is_err());
}

fn test_begin_object_key_first_false_error() {
    struct TestWriter {
        should_error: bool,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
            } else {
                Ok(buf.len())
            }
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut test_writer = TestWriter { should_error: true };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ", // Example indent
    };

    let result = formatter.begin_object_key(&mut test_writer, false);
    assert!(result.is_err());
}

