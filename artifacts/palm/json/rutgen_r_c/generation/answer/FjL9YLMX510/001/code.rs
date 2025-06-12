// Answer 0

#[test]
fn test_serialize_seq_err() {
    struct DummyWriter;
    
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct DummyFormatter;

    let writer = DummyWriter;
    let formatter = DummyFormatter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter } };

    let result = serializer.serialize_seq(None);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), key_must_be_a_string());
}

#[test]
fn test_serialize_seq_with_length_err() {
    struct DummyWriter;
    
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct DummyFormatter;

    let writer = DummyWriter;
    let formatter = DummyFormatter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter } };

    let result = serializer.serialize_seq(Some(10));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), key_must_be_a_string());
}

