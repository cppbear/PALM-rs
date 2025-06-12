// Answer 0

#[test]
fn test_serialize_u8_success() {
    struct FakeFormatter {
        output: Vec<u8>,
    }

    impl FakeFormatter {
        fn write_u8(&self, writer: &mut Vec<u8>, value: u8) -> Result<(), std::io::Error> {
            writer.push(value);
            Ok(())
        }
    }

    struct TestSerializer<'a> {
        formatter: &'a FakeFormatter,
        writer: Vec<u8>,
    }

    impl<'a> TestSerializer<'a> {
        fn serialize_u8(self, value: u8) -> Result<()> {
            self.formatter
                .write_u8(&mut self.writer, value)
                .map_err(Error::io)
        }
    }

    let formatter = FakeFormatter { output: Vec::new() };
    let serializer = TestSerializer {
        formatter: &formatter,
        writer: Vec::new(),
    };

    let result = serializer.serialize_u8(100);
    assert!(result.is_ok());
    assert_eq!(serializer.writer, vec![100]);
}

#[test]
fn test_serialize_u8_edge_case() {
    struct FakeFormatter {
        output: Vec<u8>,
    }

    impl FakeFormatter {
        fn write_u8(&self, writer: &mut Vec<u8>, value: u8) -> Result<(), std::io::Error> {
            writer.push(value);
            Ok(())
        }
    }

    struct TestSerializer<'a> {
        formatter: &'a FakeFormatter,
        writer: Vec<u8>,
    }

    impl<'a> TestSerializer<'a> {
        fn serialize_u8(self, value: u8) -> Result<()> {
            self.formatter
                .write_u8(&mut self.writer, value)
                .map_err(Error::io)
        }
    }

    let formatter = FakeFormatter { output: Vec::new() };
    let serializer = TestSerializer {
        formatter: &formatter,
        writer: Vec::new(),
    };

    let result = serializer.serialize_u8(0);
    assert!(result.is_ok());
    assert_eq!(serializer.writer, vec![0]);
}

