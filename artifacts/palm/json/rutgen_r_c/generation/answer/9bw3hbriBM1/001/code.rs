// Answer 0

#[test]
fn test_serialize_tuple() {
    struct MockWriter;
    struct MockFormatter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };

    let result = serializer.serialize_tuple(0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), key_must_be_a_string());
}

