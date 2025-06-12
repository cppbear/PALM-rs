// Answer 0

#[test]
fn test_serialize_key_with_first_state() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_object_key(
            &mut self,
            _writer: &mut dyn io::Write,
            _is_first: bool,
        ) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    struct TestKey {
        value: String,
    }

    impl Serialize for TestKey {
        fn serialize<S>(&self, serializer: S) -> Result<()>
        where
            S: ser::Serializer,
        {
            // Implementing the key serialization
            serializer.serialize_str(&self.value)
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let mut serializer = Serializer { writer, formatter };
    let state = State::First;

    let mut compound = Compound::Map { ser: &mut serializer, state: &mut state };
    let key = TestKey { value: "test_key".to_string() };

    assert!(compound.serialize_key(&key).is_ok());
}

#[test]
fn test_serialize_key_with_rest_state() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_object_key(
            &mut self,
            _writer: &mut dyn io::Write,
            _is_first: bool,
        ) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    struct TestKey {
        value: String,
    }

    impl Serialize for TestKey {
        fn serialize<S>(&self, serializer: S) -> Result<()>
        where
            S: ser::Serializer,
        {
            // Implementing the key serialization
            serializer.serialize_str(&self.value)
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let mut serializer = Serializer { writer, formatter };
    let mut state = State::First; // Start as first

    let mut compound = Compound::Map { ser: &mut serializer, state: &mut state };

    let key = TestKey { value: "test_key".to_string() };

    // First serialize the key to change the state
    assert!(compound.serialize_key(&key).is_ok());

    // Now we should be in the Rest state
    assert_eq!(state, State::Rest);

    // Test again with the state now being Rest
    assert!(compound.serialize_key(&key).is_ok());
}

