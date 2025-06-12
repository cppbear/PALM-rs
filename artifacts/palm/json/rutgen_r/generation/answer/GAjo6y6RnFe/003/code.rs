// Answer 0

#[test]
fn test_serialize_i128_success() {
    struct MockWriter;
    impl MockWriter {
        fn new() -> Self {
            MockWriter
        }
    }
    
    struct MockFormatter {
        writer: MockWriter,
    }
    
    impl MockFormatter {
        fn begin_string(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn write_i128(&self, _: &mut MockWriter, _: i128) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }
    
    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: MockFormatter {
                    writer: MockWriter::new(),
                },
                writer: MockWriter::new(),
            }
        }

        fn serialize_i128(self, value: i128) -> Result<()> {
            self.formatter
                .begin_string(&mut self.writer)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            self.formatter
                .end_string(&mut self.writer)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            Ok(())
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_i128(123456789012345678901234567890i128);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i128_failure_begin_string() {
    struct MockWriter;
    impl MockWriter {
        fn new() -> Self {
            MockWriter
        }
    }
    
    struct MockFormatter {
        writer: MockWriter,
    }
    
    impl MockFormatter {
        fn begin_string(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Error"))
        }
        
        fn write_i128(&self, _: &mut MockWriter, _: i128) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }
    
    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: MockFormatter {
                    writer: MockWriter::new(),
                },
                writer: MockWriter::new(),
            }
        }

        fn serialize_i128(self, value: i128) -> Result<()> {
            self.formatter
                .begin_string(&mut self.writer)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            self.formatter
                .end_string(&mut self.writer)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            Ok(())
        }
    }

    let serializer = Serializer::new();
    let _ = serializer.serialize_i128(123456789012345678901234567890i128);
}

#[test]
#[should_panic]
fn test_serialize_i128_failure_write_i128() {
    struct MockWriter;
    impl MockWriter {
        fn new() -> Self {
            MockWriter
        }
    }
    
    struct MockFormatter {
        writer: MockWriter,
    }
    
    impl MockFormatter {
        fn begin_string(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn write_i128(&self, _: &mut MockWriter, _: i128) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Error"))
        }

        fn end_string(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }
    
    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: MockFormatter {
                    writer: MockWriter::new(),
                },
                writer: MockWriter::new(),
            }
        }

        fn serialize_i128(self, value: i128) -> Result<()> {
            self.formatter
                .begin_string(&mut self.writer)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            self.formatter
                .end_string(&mut self.writer)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            Ok(())
        }
    }

    let serializer = Serializer::new();
    let _ = serializer.serialize_i128(123456789012345678901234567890i128);
}

#[test]
#[should_panic]
fn test_serialize_i128_failure_end_string() {
    struct MockWriter;
    impl MockWriter {
        fn new() -> Self {
            MockWriter
        }
    }
    
    struct MockFormatter {
        writer: MockWriter,
    }
    
    impl MockFormatter {
        fn begin_string(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn write_i128(&self, _: &mut MockWriter, _: i128) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Error"))
        }
    }
    
    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: MockFormatter {
                    writer: MockWriter::new(),
                },
                writer: MockWriter::new(),
            }
        }

        fn serialize_i128(self, value: i128) -> Result<()> {
            self.formatter
                .begin_string(&mut self.writer)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            self.formatter
                .end_string(&mut self.writer)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))?;
            Ok(())
        }
    }

    let serializer = Serializer::new();
    let _ = serializer.serialize_i128(123456789012345678901234567890i128);
}

