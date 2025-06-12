// Answer 0

#[test]
fn test_serialize_struct_err() {
    struct TestWriter;
    struct TestFormatter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    impl Formatter for TestFormatter {}

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: TestFormatter,
    };

    let result: Result<()> = serializer.serialize_struct("test_struct", 0);

    assert!(result.is_err());
    assert_eq!(result, Err(key_must_be_a_string()));
}

