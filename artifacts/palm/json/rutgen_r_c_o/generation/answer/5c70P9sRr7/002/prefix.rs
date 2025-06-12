// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn end_array(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let ser = Serializer {
        writer,
        formatter,
    };
    
    let compound = Compound::Map {
        ser: &mut ser,
        state: State::Empty,
    };

    let result = compound.end();
}

#[test]
fn test_end_with_non_empty_state_and_ok() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn end_array(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let ser = Serializer {
        writer,
        formatter,
    };
    
    let compound = Compound::Map {
        ser: &mut ser,
        state: State::First,
    };

    let result = compound.end();
}

#[test]
#[should_panic]
fn test_end_with_non_empty_state_and_err_on_end_array() {
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
    impl Formatter for MockFormatter {
        fn end_array(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Err(Error::new(ErrorCode::IoError))
        }
        fn end_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let ser = Serializer {
        writer,
        formatter,
    };

    let compound = Compound::Map {
        ser: &mut ser,
        state: State::First,
    };

    let result = compound.end();
}

#[test]
#[should_panic]
fn test_end_with_non_empty_state_and_err_on_end_object_value() {
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
    impl Formatter for MockFormatter {
        fn end_array(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Err(Error::new(ErrorCode::IoError))
        }
        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let ser = Serializer {
        writer,
        formatter,
    };

    let compound = Compound::Map {
        ser: &mut ser,
        state: State::First,
    };

    let result = compound.end();
}

