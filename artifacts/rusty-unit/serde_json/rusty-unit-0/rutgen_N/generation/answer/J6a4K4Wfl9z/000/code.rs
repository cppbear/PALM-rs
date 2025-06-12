// Answer 0

#[test]
fn test_serialize_i8() {
    struct MockFormatter {
        written: Vec<u8>,
    }
    
    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { written: Vec::new() }
        }
        
        fn begin_string(&mut self, writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            writer.push(b'"');
            Ok(())
        }
        
        fn write_i8(&mut self, writer: &mut Vec<u8>, value: i8) -> Result<(), std::io::Error> {
            writer.extend(value.to_string().as_bytes());
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

        fn serialize_i8(self, value: i8) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i8(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)?;
            Ok(())
        }
    }

    let serializer = MockSerializer::new();
    let result = serializer.serialize_i8(42);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "\"42\"");
}

#[test]
fn test_serialize_negative_i8() {
    struct MockFormatter {
        written: Vec<u8>,
    }
    
    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { written: Vec::new() }
        }
        
        fn begin_string(&mut self, writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            writer.push(b'"');
            Ok(())
        }
        
        fn write_i8(&mut self, writer: &mut Vec<u8>, value: i8) -> Result<(), std::io::Error> {
            writer.extend(value.to_string().as_bytes());
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

        fn serialize_i8(self, value: i8) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i8(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)?;
            Ok(())
        }
    }

    let serializer = MockSerializer::new();
    let result = serializer.serialize_i8(-42);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "\"-42\"");
}

