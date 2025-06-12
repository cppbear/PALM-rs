// Answer 0

#[test]
fn test_serialize_seq_returns_error() {
    struct TestWrite;
    impl io::Write for TestWrite {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct TestFormatter;

    let writer = TestWrite;
    let formatter = TestFormatter;
    let map_key_serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter } };

    let result = map_key_serializer.serialize_seq(None);
    assert!(result.is_err());
}

