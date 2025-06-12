// Answer 0

#[test]
fn test_serialize_i64_success() {
    struct MockFormatter {
        output: Vec<u8>,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { output: Vec::new() }
        }
        
        fn begin_string(&mut self, writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            writer.push(b'"');
            Ok(())
        }
        
        fn write_i64(&mut self, writer: &mut Vec<u8>, value: i64) -> Result<(), std::io::Error> {
            let value_string = value.to_string();
            writer.extend_from_slice(value_string.as_bytes());
            Ok(())
        }

        fn end_string(&mut self, writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            writer.push(b'"');
            Ok(())
        }
    }

    struct MockSerializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                writer: Vec::new(),
                formatter: MockFormatter::new(),
            }
        }

        fn serialize_i64(self, value: i64) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i64(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)?;
            Ok(())
        }
    }

    let serializer = MockSerializer::new();
    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "\"42\"");
}

#[test]
#[should_panic]
fn test_serialize_i64_io_failure() {
    struct FaultyFormatter;

    impl FaultyFormatter {
        fn begin_string(&mut self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "IO Error"))
        }

        fn write_i64(&mut self, _: &mut Vec<u8>, _: i64) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct FaultySerializer {
        writer: Vec<u8>,
        formatter: FaultyFormatter,
    }

    impl FaultySerializer {
        fn new() -> Self {
            FaultySerializer {
                writer: Vec::new(),
                formatter: FaultyFormatter,
            }
        }

        fn serialize_i64(self, value: i64) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i64(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)?;
            Ok(())
        }
    }

    let serializer = FaultySerializer::new();
    let _ = serializer.serialize_i64(42);
}

