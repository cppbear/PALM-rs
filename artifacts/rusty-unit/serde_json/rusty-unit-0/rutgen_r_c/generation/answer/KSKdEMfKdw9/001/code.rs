// Answer 0

#[test]
fn test_serialize_struct_variant_error() {
    struct TestWriter;
    struct TestFormatter;

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: TestFormatter,
    };

    let result = serializer.serialize_struct_variant("Test", 0, "Variant", 0);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), key_must_be_a_string());
}

