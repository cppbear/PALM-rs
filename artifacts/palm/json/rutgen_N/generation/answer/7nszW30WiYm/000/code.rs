// Answer 0

#[test]
fn test_serialize_i32_success() {
    struct TestFormatter {
        output: Vec<u8>,
    }

    impl TestFormatter {
        fn begin_string(&mut self, _: &mut Vec<u8>) -> Result<()> {
            self.output.push(b'"');
            Ok(())
        }

        fn write_i32(&mut self, _: &mut Vec<u8>, value: i32) -> Result<()> {
            self.output.extend(value.to_string().bytes());
            Ok(())
        }

        fn end_string(&mut self, _: &mut Vec<u8>) -> Result<()> {
            self.output.push(b'"');
            Ok(())
        }
    }

    struct TestSer {
        writer: Vec<u8>,
        formatter: TestFormatter,
    }

    impl TestSer {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: TestFormatter { output: Vec::new() },
            }
        }

        fn serialize_i32(self, value: i32) -> Result<()> {
            tri!(self
                .formatter
                .begin_string(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i32(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string(&mut self.writer)
                .map_err(Error::io)
        }
    }

    let ser = TestSer::new();
    let result = ser.serialize_i32(42);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i32_failure() {
    struct TestFormatterFail {
        output: Vec<u8>,
    }

    impl TestFormatterFail {
        fn begin_string(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Err(Error::io)
        }

        fn write_i32(&mut self, _: &mut Vec<u8>, _: i32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    struct TestSerFail {
        writer: Vec<u8>,
        formatter: TestFormatterFail,
    }

    impl TestSerFail {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: TestFormatterFail { output: Vec::new() },
            }
        }

        fn serialize_i32(self, value: i32) -> Result<()> {
            tri!(self
                .formatter
                .begin_string(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i32(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string(&mut self.writer)
                .map_err(Error::io)
        }
    }

    let ser = TestSerFail::new();
    let _ = ser.serialize_i32(42).expect("Should panic on failure");
}

