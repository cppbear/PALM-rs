// Answer 0

#[test]
fn test_end_map_empty_state() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn end_object(&self, _writer: &mut MockWriter) -> Result<(), ()> {
            Ok(())
        }

        fn end_object_value(&self, _writer: &mut MockWriter) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Ser {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    struct Compound {
        ser: Ser,
        state: State,
    }

    #[derive(Default)]
    enum State {
        Empty,
        #[default]
        NonEmpty,
    }

    impl Compound {
        fn end(self) -> Result<(), ()> {
            match self {
                Compound { ser, state } => {
                    match state {
                        State::Empty => {}
                        _ => ser.formatter.end_object(&mut ser.writer)?,
                    }
                    ser.formatter.end_object_value(&mut ser.writer)?;
                    ser.formatter.end_object(&mut ser.writer)
                }
            }
        }
    }

    let formatter = MockFormatter;
    let writer = MockWriter;
    let ser = Ser { formatter, writer };
    let compound = Compound { ser, state: State::Empty };

    assert!(compound.end().is_ok());
}

#[test]
fn test_end_map_non_empty_state() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn end_object(&self, _writer: &mut MockWriter) -> Result<(), ()> {
            Ok(())
        }

        fn end_object_value(&self, _writer: &mut MockWriter) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Ser {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    struct Compound {
        ser: Ser,
        state: State,
    }

    #[derive(Default)]
    enum State {
        Empty,
        NonEmpty,
    }

    impl Compound {
        fn end(self) -> Result<(), ()> {
            match self {
                Compound { ser, state } => {
                    match state {
                        State::Empty => {}
                        _ => ser.formatter.end_object(&mut ser.writer)?,
                    }
                    ser.formatter.end_object_value(&mut ser.writer)?;
                    ser.formatter.end_object(&mut ser.writer)
                }
            }
        }
    }

    let formatter = MockFormatter;
    let writer = MockWriter;
    let ser = Ser { formatter, writer };
    let compound = Compound { ser, state: State::NonEmpty };

    assert!(compound.end().is_ok());
}

