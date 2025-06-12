// Answer 0

fn test_serialize_f32_valid() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_f32(1.0);
    assert!(result.is_ok());
}

fn test_serialize_f32_infinite() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_f32(f32::INFINITY);
    assert!(result.is_err());
}

fn test_serialize_f32_nan() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_f32(f32::NAN);
    assert!(result.is_err());
}

