// Answer 0

#[test]
fn test_serialize_map_err() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = TestWriter;
    let serializer = Serializer::<_, TestFormatter> {
        writer,
        formatter: TestFormatter,
    };
    
    let result = serializer.serialize_map(None);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorCode::KeyMustBeAString);
}

