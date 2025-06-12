// Answer 0

#[test]
fn test_end_serialize_map() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Empty,
    };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_end_arbitrary_precision() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    let compound = Compound::Number {
        ser: &mut Serializer { writer, formatter },
    };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
#[cfg(feature = "raw_value")]
fn test_end_raw_value() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    let compound = Compound::RawValue {
        ser: &mut Serializer { writer, formatter },
    };

    let result = compound.end();
    assert!(result.is_ok());
}

