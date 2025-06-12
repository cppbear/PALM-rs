// Answer 0

#[test]
fn test_serialize_map_with_none_length() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct DummyFormatter;

    let writer = DummyWriter;
    let formatter = DummyFormatter;
    let serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer,
            formatter
        },
    };
    
    let result = serializer.serialize_map(None);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), key_must_be_a_string());
}

#[test]
fn test_serialize_map_with_some_length() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct DummyFormatter;

    let writer = DummyWriter;
    let formatter = DummyFormatter;
    let serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer,
            formatter
        },
    };
    
    let result = serializer.serialize_map(Some(5));
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), key_must_be_a_string());
}

