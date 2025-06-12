// Answer 0

#[test]
fn test_serialize_unit_struct() {
    struct MockWriter;
    struct MockFormatter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter } };

    let result = serializer.serialize_unit_struct("test_name");
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, key_must_be_a_string());
    }
}

