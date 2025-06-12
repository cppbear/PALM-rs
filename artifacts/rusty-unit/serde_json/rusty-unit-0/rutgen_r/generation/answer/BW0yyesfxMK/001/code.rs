// Answer 0

#[test]
fn test_serialize_i128_success() {
    struct Formatter;
    struct Writer(Vec<u8>);

    impl Formatter {
        fn write_i128(&self, writer: &mut Writer, value: i128) -> Result<(), std::io::Error> {
            let result = value.to_string().into_bytes();
            writer.0.extend(result);
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Writer,
    }

    impl Serializer {
        fn new(writer: Writer) -> Self {
            Self {
                formatter: Formatter,
                writer,
            }
        }

        fn serialize_i128(self, value: i128) -> Result<(), std::io::Error> {
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let writer = Writer(Vec::new());
    let serializer = Serializer::new(writer);
    
    let result = serializer.serialize_i128(123456789012345678901234567890);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i128_zero() {
    struct Formatter;
    struct Writer(Vec<u8>);

    impl Formatter {
        fn write_i128(&self, writer: &mut Writer, value: i128) -> Result<(), std::io::Error> {
            let result = value.to_string().into_bytes();
            writer.0.extend(result);
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Writer,
    }

    impl Serializer {
        fn new(writer: Writer) -> Self {
            Self {
                formatter: Formatter,
                writer,
            }
        }

        fn serialize_i128(self, value: i128) -> Result<(), std::io::Error> {
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let writer = Writer(Vec::new());
    let serializer = Serializer::new(writer);
    
    let result = serializer.serialize_i128(0);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i128_negative() {
    struct Formatter;
    struct Writer(Vec<u8>);

    impl Formatter {
        fn write_i128(&self, writer: &mut Writer, value: i128) -> Result<(), std::io::Error> {
            let result = value.to_string().into_bytes();
            writer.0.extend(result);
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Writer,
    }

    impl Serializer {
        fn new(writer: Writer) -> Self {
            Self {
                formatter: Formatter,
                writer,
            }
        }

        fn serialize_i128(self, value: i128) -> Result<(), std::io::Error> {
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let writer = Writer(Vec::new());
    let serializer = Serializer::new(writer);
    
    let result = serializer.serialize_i128(-123456789012345678901234567890);
    assert!(result.is_ok());
}

