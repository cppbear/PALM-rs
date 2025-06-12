// Answer 0

fn test_serialize_seq_empty_array() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn begin_array(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_array(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        Empty,
        First,
    }

    enum Compound {
        Map { ser: Serializer, state: State },
    }

    impl Serializer {
        fn serialize_seq(self, len: Option<usize>) -> Result<Compound, std::io::Error> {
            self.formatter.begin_array(&mut self.writer)?;
            if len == Some(0) {
                self.formatter.end_array(&mut self.writer)?;
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
        formatter: MockFormatter,
        writer: MockWriter,
    };
    
    let result = serializer.serialize_seq(Some(0)).unwrap();
    
    match result {
        Compound::Map { ser, state } => {
            assert_eq!(state, State::Empty);
            assert!(std::ptr::eq(&ser, &serializer));
        }
    }
}

fn test_serialize_seq_non_empty_array() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn begin_array(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_array(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        Empty,
        First,
    }

    enum Compound {
        Map { ser: Serializer, state: State },
    }

    impl Serializer {
        fn serialize_seq(self, len: Option<usize>) -> Result<Compound, std::io::Error> {
            self.formatter.begin_array(&mut self.writer)?;
            if len == Some(0) {
                self.formatter.end_array(&mut self.writer)?;
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
        formatter: MockFormatter,
        writer: MockWriter,
    };
    
    let result = serializer.serialize_seq(Some(1)).unwrap();

    match result {
        Compound::Map { ser, state } => {
            assert_eq!(state, State::First);
            assert!(std::ptr::eq(&ser, &serializer));
        }
    }
}

