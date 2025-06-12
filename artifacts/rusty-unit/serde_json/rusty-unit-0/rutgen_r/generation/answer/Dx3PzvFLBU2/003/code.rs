// Answer 0

#[test]
fn test_serialize_map_empty() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_object(&self, _writer: &mut ()) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object(&self, _writer: &mut ()) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: MockFormatter,
        writer: (),
    }

    impl TestSerializer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound<TestSerializer>> {
            self.formatter.begin_object(&mut self.writer)?;
            if len == Some(0) {
                self.formatter.end_object(&mut self.writer)?;
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

    enum Compound<S> {
        Map { ser: S, state: State },
    }

    enum State {
        Empty,
        First,
    }

    let serializer = TestSerializer {
        formatter: MockFormatter,
        writer: (),
    };

    let result = serializer.serialize_map(Some(0)).unwrap();
    match result {
        Compound::Map { ser, state } => {
            assert!(matches!(state, State::Empty));
        }
    }
}

