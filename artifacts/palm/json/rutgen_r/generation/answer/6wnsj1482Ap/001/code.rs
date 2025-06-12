// Answer 0

#[test]
fn test_serialize_u128_success() {
    struct TestWriter {
        output: Vec<u8>
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }

        fn into_vec(self) -> Vec<u8> {
            self.output
        }
    }

    struct Formatter;

    impl Formatter {
        fn write_u128(&self, writer: &mut TestWriter, value: u128) -> Result<(), std::io::Error> {
            let serialized = value.to_string();
            writer.output.extend_from_slice(serialized.as_bytes());
            Ok(())
        }
    }

    struct Serializer {
        writer: TestWriter,
        formatter: Formatter,
    }

    let mut writer = TestWriter::new();
    let formatter = Formatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_u128(1234567890123456789012345678901234567890);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.into_vec(), b"1234567890123456789012345678901234567890");
}

#[test]
#[should_panic]
fn test_serialize_u128_panic() {
    struct FailingFormatter;

    impl FailingFormatter {
        fn write_u128(&self, _writer: &mut TestWriter, _value: u128) -> Result<(), std::io::Error> {
            panic!("Intentional panic for testing");
        }
    }

    struct FailingSerializer {
        writer: TestWriter,
        formatter: FailingFormatter,
    }

    let mut writer = TestWriter::new();
    let formatter = FailingFormatter;
    let serializer = FailingSerializer { writer, formatter };

    // This should cause a panic.
    let _ = serializer.serialize_u128(1234567890123456789012345678901234567890);
}

