// Answer 0

#[test]
fn test_end_with_empty_state() {
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
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    
    let compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Empty,
    };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
fn test_end_with_non_empty_state() {
    struct MockWriter {
        ended: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        should_end: bool,
    }

    impl Formatter for MockFormatter {
        fn end_array(&mut self, _: &mut dyn io::Write) -> Result<()> {
            if self.should_end {
                Ok(())
            } else {
                Err(Error::new(ErrorCode::Invalid))
            }
        }
    }

    let writer = MockWriter { ended: false };
    let formatter = MockFormatter { should_end: true };

    let compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Rest,
    };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_with_invalid_state() {
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
            panic!("Should not reach here when state is empty");
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;

    let compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Empty,
    };

    let _ = compound.end(); // This should panic
}

