// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockWriter;
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter
        }

        fn write(&mut self, _data: &[u8]) -> Result<usize, std::io::Error> {
            Ok(data.len())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter
        }
        
        fn end_object(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                writer: MockWriter::new(),
                formatter: MockFormatter::new(),
            }
        }
    }

    enum State {
        Empty,
        NonEmpty,
    }

    enum Compound {
        Map { ser: MockSerializer, state: State },
    }

    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => match state {
                State::Empty => Ok(()),
                _ => ser.formatter.end_object(&mut ser.writer).map_err(Error::io),
            },
        }
    }

    let serializer = MockSerializer::new();
    let compound = Compound::Map {
        ser: serializer,
        state: State::Empty,
    };

    let result = end(compound);
    assert_eq!(result, Ok(()));
}

