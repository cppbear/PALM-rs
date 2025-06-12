// Answer 0

#[test]
fn test_serialize_map_with_length_zero_and_error_in_begin_object() {
    struct MockWriter;

    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn begin_object(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_object failed"))
            } else {
                Ok(())
            }
        }

        fn end_object(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct TestSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl TestSer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound<TestSer>, std::io::Error> {
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

    let formatter = MockFormatter { should_fail: true };
    let writer = MockWriter;
    let ser = TestSer { formatter, writer };

    let result = ser.serialize_map(Some(0));
    assert!(result.is_err());
}

#[test]
fn test_serialize_map_with_length_zero_and_successful_begin() {
    struct MockWriter;

    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn begin_object(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct TestSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl TestSer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound<TestSer>, std::io::Error> {
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

    let formatter = MockFormatter { should_fail: false };
    let writer = MockWriter;
    let ser = TestSer { formatter, writer };

    let result = ser.serialize_map(Some(0));
    assert!(result.is_ok());
}

