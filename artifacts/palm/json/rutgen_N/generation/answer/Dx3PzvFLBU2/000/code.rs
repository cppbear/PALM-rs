// Answer 0

#[test]
fn test_serialize_map_empty() {
    struct Formatter;
    struct Writer;

    impl Formatter {
        fn begin_object(&self, _: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object(&self, _: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Writer,
    }

    impl Serializer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound> {
            tri!(self.formatter.begin_object(&mut self.writer).map_err(Error::io));
            if len == Some(0) {
                tri!(self.formatter.end_object(&mut self.writer).map_err(Error::io));
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

    struct Compound {
        ser: Serializer,
        state: State,
    }

    enum State {
        Empty,
        First,
    }

    struct Error;

    impl Error {
        fn io(_: std::io::Error) -> Self {
            Error
        }
    }

    let serializer = Serializer {
        formatter: Formatter,
        writer: Writer,
    };
    
    let result = serializer.serialize_map(Some(0));
    match result {
        Ok(Compound { state, .. }) => assert_eq!(state, State::Empty),
        Err(_) => panic!("Serialization failed for an empty map"),
    }
}

#[test]
fn test_serialize_map_non_empty() {
    struct Formatter;
    struct Writer;

    impl Formatter {
        fn begin_object(&self, _: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object(&self, _: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Writer,
    }

    impl Serializer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound> {
            tri!(self.formatter.begin_object(&mut self.writer).map_err(Error::io));
            if len == Some(0) {
                tri!(self.formatter.end_object(&mut self.writer).map_err(Error::io));
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

    struct Compound {
        ser: Serializer,
        state: State,
    }

    enum State {
        Empty,
        First,
    }

    struct Error;

    impl Error {
        fn io(_: std::io::Error) -> Self {
            Error
        }
    }

    let serializer = Serializer {
        formatter: Formatter,
        writer: Writer,
    };
    
    let result = serializer.serialize_map(Some(1));
    match result {
        Ok(Compound { state, .. }) => assert_eq!(state, State::First),
        Err(_) => panic!("Serialization failed for a non-empty map"),
    }
}

