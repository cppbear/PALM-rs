// Answer 0

#[test]
fn test_serialize_key_valid_writer_state_first_non_serializable_key() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(0)
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct NonSerializable;

    impl Serialize for NonSerializable {
        fn serialize<S>(&self, _: S) -> Result<()>
        where
            S: Serializer,
        {
            Err(Error::from(ErrorCode::NotSerializable))
        }
    }

    let mut writer = TestWriter;
    let formatter = CompactFormatter;
    let mut state = State::First;
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: &mut state,
    };
    
    let key = NonSerializable;

    let result = compound.serialize_key(&key);
}

#[test]
fn test_serialize_key_writer_error_state_first() {
    struct FaultyWriter;

    impl io::Write for FaultyWriter {
        fn write(&mut self, _: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct SerializableKey;

    impl Serialize for SerializableKey {
        fn serialize<S>(&self, _: S) -> Result<()>
        where
            S: Serializer,
        {
            Ok(())
        }
    }

    let mut writer = FaultyWriter;
    let formatter = CompactFormatter;
    let mut state = State::First;
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: &mut state,
    };

    let key = SerializableKey;

    let result = compound.serialize_key(&key);
}

