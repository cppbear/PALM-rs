// Answer 0

#[test]
fn test_serialize_u8_success() {
    struct Formatter;
    
    impl Formatter {
        fn write_u8(&self, writer: &mut Vec<u8>, value: u8) -> std::io::Result<()> {
            writer.push(value);
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn new(formatter: Formatter) -> Self {
            Serializer {
                formatter,
                writer: Vec::new(),
            }
        }
    
        fn serialize_u8(self, value: u8) -> Result<(), std::io::Error> {
            self.formatter
                .write_u8(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let formatter = Formatter;
    let mut serializer = Serializer::new(formatter);
    let result = serializer.serialize_u8(42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer, vec![42]);
}

#[test]
#[should_panic]
fn test_serialize_u8_io_error() {
    struct ErrorFormatter;

    impl ErrorFormatter {
        fn write_u8(&self, _writer: &mut Vec<u8>, _value: u8) -> std::io::Result<()> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Forced error"))
        }
    }

    struct Serializer {
        formatter: ErrorFormatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn new(formatter: ErrorFormatter) -> Self {
            Serializer {
                formatter,
                writer: Vec::new(),
            }
        }
    
        fn serialize_u8(self, value: u8) -> Result<(), std::io::Error> {
            self.formatter
                .write_u8(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let formatter = ErrorFormatter;
    let serializer = Serializer::new(formatter);
    let _ = serializer.serialize_u8(42).unwrap();  // This should panic
}

