// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    let mut serializer = MapKeySerializer {
        ser: &mut Serializer::new(MockWriter),
    };
    let result = serializer.serialize_bytes(&[]);
    assert_eq!(result, Err(key_must_be_a_string()));
}

#[test]
fn test_serialize_bytes_non_empty() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = MapKeySerializer {
        ser: &mut Serializer::new(MockWriter),
    };
    let result = serializer.serialize_bytes(&[1, 2, 3]);
    assert_eq!(result, Err(key_must_be_a_string()));
}

#[test]
fn test_serialize_bytes_large_input() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = MapKeySerializer {
        ser: &mut Serializer::new(MockWriter),
    };
    let result = serializer.serialize_bytes(&[0; 1024]);
    assert_eq!(result, Err(key_must_be_a_string()));
}

