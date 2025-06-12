// Answer 0

#[test]
fn test_serialize_seq_non_zero_length() {
    struct MockFormatter;
    struct MockWriter;
    
    impl MockFormatter {
        fn begin_array(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn end_array(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }
    
    struct MockSerializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        First,
        Empty,
    }

    enum Compound {
        Map {
            ser: MockSerializer,
            state: State,
        }
    }

    impl MockSerializer {
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

    let serializer = MockSerializer {
        formatter: MockFormatter,
        writer: MockWriter,
    };

    let result = serializer.serialize_seq(Some(1));
    assert!(result.is_ok());
    if let Ok(compound) = result {
        match compound {
            Compound::Map { ser, state } => {
                assert_eq!(state, State::First);
            }
        }
    }
}

