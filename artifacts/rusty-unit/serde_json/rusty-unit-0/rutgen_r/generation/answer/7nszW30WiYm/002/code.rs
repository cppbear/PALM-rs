// Answer 0

#[test]
fn test_serialize_i32_success() {
    struct MockWriter;
    struct MockFormatter {
        write_called: bool,
        write_value: Option<i32>,
    }
    
    struct MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter
        }
    }
    
    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                write_called: false,
                write_value: None,
            }
        }
        
        fn begin_string(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn write_i32(&mut self, _writer: &mut MockWriter, value: i32) -> Result<(), std::io::Error> {
            self.write_called = true;
            self.write_value = Some(value);
            Ok(())
        }
        
        fn end_string(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    let mut formatter = MockFormatter::new();
    let serializer = MockSerializer {
        writer: MockWriter::new(),
        formatter,
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_ok());
    assert!(serializer.formatter.write_called);
    assert_eq!(serializer.formatter.write_value, Some(42));
}

#[test]
fn test_serialize_i32_begin_string_error() {
    struct MockErrorWriter;
    struct MockErrorFormatter;

    struct MockErrorSerializer {
        writer: MockErrorWriter,
        formatter: MockErrorFormatter,
    }

    impl MockErrorWriter {
        fn new() -> Self {
            MockErrorWriter
        }
    }

    impl MockErrorFormatter {
        fn new() -> Self {
            MockErrorFormatter
        }
        
        fn begin_string(&self, _writer: &mut MockErrorWriter) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Begin string error"))
        }

        fn write_i32(&self, _writer: &mut MockErrorWriter, _value: i32) -> Result<(), std::io::Error> {
            unreachable!();
        }

        fn end_string(&self, _writer: &mut MockErrorWriter) -> Result<(), std::io::Error> {
            unreachable!();
        }
    }

    let serializer = MockErrorSerializer {
        writer: MockErrorWriter::new(),
        formatter: MockErrorFormatter::new(),
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_i32_write_i32_error() {
    struct MockErrorWriter;
    struct MockErrorFormatter {
        is_write_error: bool,
    }

    struct MockErrorSerializer {
        writer: MockErrorWriter,
        formatter: MockErrorFormatter,
    }

    impl MockErrorWriter {
        fn new() -> Self {
            MockErrorWriter
        }
    }

    impl MockErrorFormatter {
        fn new() -> Self {
            MockErrorFormatter {
                is_write_error: true,
            }
        }
        
        fn begin_string(&self, _writer: &mut MockErrorWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_i32(&mut self, _writer: &mut MockErrorWriter, _value: i32) -> Result<(), std::io::Error> {
            if self.is_write_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Write i32 error"))
            } else {
                Ok(())
            }
        }

        fn end_string(&self, _writer: &mut MockErrorWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    let serializer = MockErrorSerializer {
        writer: MockErrorWriter::new(),
        formatter: MockErrorFormatter::new(),
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_err());
}

