// Answer 0

#[test]
fn test_serialize_key_success() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_object_key(&mut self, _writer: &mut MockWriter, _is_first: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }
    
    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let mut state = State::First;
    
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: &mut state,
    };
    
    let result = compound.serialize_key::<String>(&"key_value".to_string());

    assert!(result.is_ok());
}

#[test]
fn test_serialize_key_error_on_key_serialize() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_object_key(&mut self, _writer: &mut MockWriter, _is_first: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    struct FailingKey;
    
    impl Serialize for FailingKey {
        fn serialize<S>(&self, _serializer: S) -> Result where S: ser::Serializer {
            Err(Error::new())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let mut state = State::First;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: &mut state,
    };
    
    let result = compound.serialize_key::<&FailingKey>(&FailingKey);
    
    assert!(result.is_err());
}

