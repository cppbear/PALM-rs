// Answer 0

#[test]
fn test_serialize_unit_struct() {
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

    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer: TestWriter,
            formatter: TestFormatter,
        },
    };
    
    let result = serializer.serialize_unit_struct("TestStruct");
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, key_must_be_a_string());
    }
}

