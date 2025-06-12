// Answer 0

#[test]
fn test_end_on_empty_map() {
    struct MockFormatter;
    
    impl MockFormatter {
        fn end_array(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object_value(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    enum State {
        Empty,
        NonEmpty,
    }

    enum Compound {
        Map { ser: Serializer, state: State },
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Compound {
        fn end(self) -> Result<(), Error> {
            match self {
                Compound::Map { ser, state } => {
                    match state {
                        State::Empty => {}
                        _ => tri!(ser.formatter.end_array(&mut ser.writer).map_err(Error::io)),
                    }
                    tri!(ser
                        .formatter
                        .end_object_value(&mut ser.writer)
                        .map_err(Error::io));
                    ser.formatter.end_object(&mut ser.writer).map_err(Error::io)
                }
            }
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { formatter, writer };
    let compound = Compound::Map { ser: serializer, state: State::Empty };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
fn test_end_on_non_empty_map() {
    struct MockFormatter;
    
    impl MockFormatter {
        fn end_array(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object_value(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    enum State {
        Empty,
        NonEmpty,
    }

    enum Compound {
        Map { ser: Serializer, state: State },
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Compound {
        fn end(self) -> Result<(), Error> {
            match self {
                Compound::Map { ser, state } => {
                    match state {
                        State::Empty => {}
                        _ => tri!(ser.formatter.end_array(&mut ser.writer).map_err(Error::io)),
                    }
                    tri!(ser
                        .formatter
                        .end_object_value(&mut ser.writer)
                        .map_err(Error::io));
                    ser.formatter.end_object(&mut ser.writer).map_err(Error::io)
                }
            }
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { formatter, writer };
    let compound = Compound::Map { ser: serializer, state: State::NonEmpty };

    let result = compound.end();
    assert!(result.is_ok());
}

