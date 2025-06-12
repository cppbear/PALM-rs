// Answer 0

fn test_end_object_success() -> std::io::Result<()> {
    struct Writer {
        buffer: Vec<u8>,
    }

    impl std::io::Write for Writer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = Writer { buffer: Vec::new() };
    let mut serializer = Serializer {
        current_indent: 1,
        indent: 2,
        has_value: true,
    };

    serializer.end_object(&mut writer)?;

    assert_eq!(writer.buffer, b"\n  }");
    Ok(())
}

#[test]
fn test_end_object_indent_error() {
    struct ErrWriter;

    impl std::io::Write for ErrWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    
    let mut writer = ErrWriter;
    let mut serializer = Serializer {
        current_indent: 1,
        indent: 2,
        has_value: true,
    };

    // Simulating a panic in `indent`
    let result = serializer.end_object(&mut writer);
    assert!(result.is_err());
}

#[test]
fn test_end_object_no_value() {
    struct Writer {
        buffer: Vec<u8>,
    }

    impl std::io::Write for Writer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    
    let mut writer = Writer { buffer: Vec::new() };
    let mut serializer = Serializer {
        current_indent: 1,
        indent: 2,
        has_value: false,
    };

    let result = serializer.end_object(&mut writer);
    assert!(result.is_ok());
    assert_eq!(writer.buffer, b"}");
}

