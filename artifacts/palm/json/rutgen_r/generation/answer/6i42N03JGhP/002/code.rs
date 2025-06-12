// Answer 0

fn serialize_u128(self, value: u128) -> Result<()> {
    tri!(self
        .ser
        .formatter
        .begin_string(&mut self.ser.writer)
        .map_err(Error::io));
    tri!(self
        .ser
        .formatter
        .write_u128(&mut self.ser.writer, value)
        .map_err(Error::io));
    self.ser
        .formatter
        .end_string(&mut self.ser.writer)
        .map_err(Error::io)
}

#[test]
fn test_serialize_u128_success() {
    struct MockFormatter {
        begin_called: bool,
        write_called: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                begin_called: false,
                write_called: false,
            }
        }
    }

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut Writer) -> Result<()> {
            self.begin_called = true;
            Ok(())
        }

        fn write_u128(&mut self, _writer: &mut Writer, _value: u128) -> Result<()> {
            self.write_called = true;
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Writer) -> Result<()> {
            assert!(self.begin_called);
            assert!(self.write_called);
            Ok(())
        }
    }

    struct MockSer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSer {
        fn new() -> Self {
            MockSer {
                writer: Vec::new(),
                formatter: MockFormatter::new(),
            }
        }
    }

    let mock_ser = MockSer::new();
    let result = serialize_u128(mock_ser, 1u128);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_u128_write_failure() {
    struct MockFormatter {
        begin_called: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                begin_called: false,
            }
        }
    }

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut Writer) -> Result<()> {
            self.begin_called = true;
            Ok(())
        }

        fn write_u128(&mut self, _writer: &mut Writer, _value: u128) -> Result<()> {
            Err(Error::io)
        }

        fn end_string(&mut self, _writer: &mut Writer) -> Result<()> {
            panic!("end_string should not be called");
        }
    }

    struct MockSer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSer {
        fn new() -> Self {
            MockSer {
                writer: Vec::new(),
                formatter: MockFormatter::new(),
            }
        }
    }

    let mock_ser = MockSer::new();
    serialize_u128(mock_ser, 1u128).unwrap(); // This should panic
}

#[test]
#[should_panic]
fn test_serialize_u128_begin_failure() {
    struct MockFormatter;

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {}
        }
    }

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut Writer) -> Result<()> {
            Err(Error::io)
        }

        fn write_u128(&mut self, _writer: &mut Writer, _value: u128) -> Result<()> {
            panic!("write_u128 should not be called");
        }

        fn end_string(&mut self, _writer: &mut Writer) -> Result<()> {
            panic!("end_string should not be called");
        }
    }

    struct MockSer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSer {
        fn new() -> Self {
            MockSer {
                writer: Vec::new(),
                formatter: MockFormatter::new(),
            }
        }
    }

    let mock_ser = MockSer::new();
    serialize_u128(mock_ser, 1u128).unwrap(); // This should panic
}

