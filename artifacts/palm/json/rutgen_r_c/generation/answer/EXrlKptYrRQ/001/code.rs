// Answer 0

#[test]
fn test_serialize_none_should_return_err_key_must_be_a_string() {
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

    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer: TestWriter,
            formatter: TestFormatter,
        },
    };

    let result = serializer.serialize_none();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), key_must_be_a_string());
}

