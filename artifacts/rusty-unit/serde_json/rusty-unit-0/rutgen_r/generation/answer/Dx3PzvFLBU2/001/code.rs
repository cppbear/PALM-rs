// Answer 0

fn test_serialize_map_err() -> Result<(), Box<dyn std::error::Error>> {
    struct DummyWriter;
    
    impl DummyWriter {
        fn new() -> Self {
            DummyWriter
        }
    }

    struct DummyFormatter;

    impl DummyFormatter {
        fn begin_object(&self, _writer: &mut DummyWriter) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "mock error"))
        }
        
        fn end_object(&self, _writer: &mut DummyWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        writer: DummyWriter,
        formatter: DummyFormatter,
    }

    impl Serializer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound, Error> {
            tri!(self
                .formatter
                .begin_object(&mut self.writer)
                .map_err(Error::io));
            if len == Some(0) {
                tri!(self
                    .formatter
                    .end_object(&mut self.writer)
                    .map_err(Error::io));
                Ok(Compound::Map {
                    ser: self,
                    state: State::Empty,
                })
            } else {
                Ok(Compound::Map {
                    ser: self,
                    state: State::First,
                })
            }
        }
    }

    let serializer = Serializer {
        writer: DummyWriter::new(),
        formatter: DummyFormatter,
    };

    let result = serializer.serialize_map(None);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_serialize_map_returns_err() {
    let result = test_serialize_map_err();
    assert!(result.is_ok());
}

