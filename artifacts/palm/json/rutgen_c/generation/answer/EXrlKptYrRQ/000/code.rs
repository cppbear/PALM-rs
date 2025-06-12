// Answer 0

#[test]
fn test_serialize_none() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct MockFormatter;

    let mut mock_writer = MockWriter;
    let serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer: mock_writer,
            formatter: MockFormatter,
        },
    };
    
    let result = serializer.serialize_none();
    assert!(result.is_err());
}

#[test]
fn test_serialize_some() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct MockFormatter;

    let mut mock_writer = MockWriter;
    let serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer: mock_writer,
            formatter: MockFormatter,
        },
    };
    
    let err = serializer.serialize_some(&"value").unwrap_err();
    assert_eq!(err, key_must_be_a_string());
}

