// Answer 0

#[test]
fn test_serialize_u8_success() {
    struct MockWriter {
        pub data: Vec<u8>,
    }

    struct MockFormatter {
        pub writer: MockWriter,
    }

    struct MockSer {
        pub formatter: MockFormatter,
    }

    struct Serializer {
        pub ser: MockSer,
    }

    impl MockWriter {
        pub fn new() -> Self {
            MockWriter { data: Vec::new() }
        }
    }

    impl MockFormatter {
        pub fn new(writer: MockWriter) -> Self {
            MockFormatter { writer }
        }

        pub fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        pub fn write_u8(&mut self, _: &mut MockWriter, value: u8) -> Result<(), std::io::Error> {
            if value > 100 {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "value too large"))
            } else {
                self.writer.data.push(value);
                Ok(())
            }
        }

        pub fn end_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl Serializer {
        pub fn serialize_u8(self, value: u8) -> Result<(), std::io::Error> {
            self.ser.formatter.begin_string(&mut self.ser.writer).map_err(|e| e)?;
            self.ser.formatter.write_u8(&mut self.ser.writer, value).map_err(|e| e)?;
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(|e| e)
        }
    }

    let writer = MockWriter::new();
    let formatter = MockFormatter::new(writer);
    let ser = MockSer { formatter };
    let serializer = Serializer { ser };

    // Test for successful serialization
    let result = serializer.serialize_u8(50);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u8_error_on_write() {
    struct MockWriter {
        pub data: Vec<u8>,
    }

    struct MockFormatter {
        pub writer: MockWriter,
    }

    struct MockSer {
        pub formatter: MockFormatter,
    }

    struct Serializer {
        pub ser: MockSer,
    }

    impl MockWriter {
        pub fn new() -> Self {
            MockWriter { data: Vec::new() }
        }
    }

    impl MockFormatter {
        pub fn new(writer: MockWriter) -> Self {
            MockFormatter { writer }
        }

        pub fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        pub fn write_u8(&mut self, _: &mut MockWriter, value: u8) -> Result<(), std::io::Error> {
            if value > 100 {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "value too large"))
            } else {
                self.writer.data.push(value);
                Ok(())
            }
        }

        pub fn end_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl Serializer {
        pub fn serialize_u8(self, value: u8) -> Result<(), std::io::Error> {
            self.ser.formatter.begin_string(&mut self.ser.writer).map_err(|e| e)?;
            self.ser.formatter.write_u8(&mut self.ser.writer, value).map_err(|e| e)?;
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(|e| e)
        }
    }

    let writer = MockWriter::new();
    let formatter = MockFormatter::new(writer);
    let ser = MockSer { formatter };
    let serializer = Serializer { ser };

    // Test for error on write
    let result = serializer.serialize_u8(101);
    assert!(result.is_err());
}

