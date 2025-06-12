// Answer 0

#[test]
fn test_serialize_u32_success() {
    struct MockFormatter;
    impl MockFormatter {
        fn write_u32(&self, writer: &mut Vec<u8>, value: u32) -> Result<(), std::io::Error> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer<'a> {
        formatter: MockFormatter,
        writer: &'a mut Vec<u8>,
    }

    fn serialize_u32<'a>(self: Serializer<'a>, value: u32) -> Result<(), std::io::Error> {
        self.formatter
            .write_u32(self.writer, value)
            .map_err(|e| e)
    }

    let mut writer = Vec::new();
    let serializer = Serializer {
        formatter: MockFormatter,
        writer: &mut writer,
    };

    let result = serialize_u32(serializer, 42);
    assert!(result.is_ok());
    assert_eq!(writer, vec![42, 0, 0, 0]);
}

#[test]
#[should_panic]
fn test_serialize_u32_failure() {
    struct FailingFormatter;
    impl FailingFormatter {
        fn write_u32(&self, _: &mut Vec<u8>, _: u32) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Failing formatter"))
        }
    }

    struct FailingSerializer<'a> {
        formatter: FailingFormatter,
        writer: &'a mut Vec<u8>,
    }

    fn serialize_u32<'a>(self: FailingSerializer<'a>, value: u32) -> Result<(), std::io::Error> {
        self.formatter
            .write_u32(self.writer, value)
            .map_err(|e| e)
    }

    let mut writer = Vec::new();
    let serializer = FailingSerializer {
        formatter: FailingFormatter,
        writer: &mut writer,
    };

    let _ = serialize_u32(serializer, 100); // This should panic
}

