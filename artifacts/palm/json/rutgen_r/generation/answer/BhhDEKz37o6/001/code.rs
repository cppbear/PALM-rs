// Answer 0

#[test]
fn test_end_with_non_empty_state_and_io_error() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn end_object(&mut self, _writer: &mut ()) -> Result<()> {
            if self.should_fail {
                Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "IO error")))
            } else {
                Ok(())
            }
        }

        fn end_object_value(&mut self, _writer: &mut ()) -> Result<()> {
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: (),
    }

    enum State {
        NonEmpty,
        Empty,
    }

    enum Compound {
        Map { ser: MockSerializer, state: State },
    }

    impl Compound {
        fn end(self) -> Result<()> {
            match self {
                Compound::Map { ser, state } => {
                    match state {
                        State::Empty => {}
                        _ => ser.formatter.end_object(&mut ser.writer).map_err(Error::io)?,
                    }
                    ser.formatter.end_object_value(&mut ser.writer).map_err(Error::io)?;
                    ser.formatter.end_object(&mut ser.writer).map_err(Error::io)
                }
            }
        }
    }

    let formatter = MockFormatter { should_fail: true };
    let writer = ();
    let serializer = MockSerializer { formatter, writer };
    let compound = Compound::Map { ser: serializer, state: State::NonEmpty };

    let result = compound.end();
    assert!(result.is_err());
}

