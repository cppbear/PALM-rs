// Answer 0

#[test]
fn test_begin_object_valid_writer() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let result = formatter.begin_object(&mut writer);
    assert!(result.is_ok());
    assert_eq!(formatter.current_indent, 1);
    assert!(!formatter.has_value);
    assert_eq!(writer.buffer, b"{");
}

#[test]
fn test_begin_object_empty_writer() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let result = formatter.begin_object(&mut writer);
    assert!(result.is_ok());
    assert_eq!(formatter.current_indent, 1);
    assert!(!formatter.has_value);
}

#[test]
#[should_panic]
fn test_begin_object_panic_when_writer_write_fails() {
    struct PanicWriter;

    impl io::Write for PanicWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "mock error"))
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = PanicWriter;
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let _ = formatter.begin_object(&mut writer); // should panic
}

