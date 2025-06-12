// Answer 0

#[test]
fn test_serialize_i128_begin_string_error() {
    struct MockWriter {
        should_fail: bool,
    }

    impl MockWriter {
        fn new(should_fail: bool) -> Self {
            MockWriter { should_fail }
        }
    }

    struct MockFormatter<'a> {
        writer: &'a mut MockWriter,
    }

    impl<'a> MockFormatter<'a> {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            if self.writer.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string failed"))
            } else {
                Ok(())
            }
        }

        fn write_i128(&mut self, _: &mut MockWriter, _: i128) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSerializer<'a> {
        formatter: MockFormatter<'a>,
    }

    impl<'a> MockSerializer<'a> {
        fn new(writer: &'a mut MockWriter) -> Self {
            MockSerializer {
                formatter: MockFormatter { writer },
            }
        }
    }

    struct TestSer {
        ser: MockSerializer<'static>,
    }

    impl TestSer {
        fn new(writer: &'static mut MockWriter) -> Self {
            TestSer {
                ser: MockSerializer::new(writer),
            }
        }

        fn serialize_i128(self, value: i128) -> Result<()> {
            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(Error::io));
            tri!(self.ser.formatter.write_i128(&mut self.ser.writer, value).map_err(Error::io));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
        }
    }

    let mut writer = MockWriter::new(true);
    let test_ser = TestSer::new(&mut writer);
    
    let result = test_ser.serialize_i128(12345678901234567890i128);
    
    assert!(result.is_err());
}

#[test]
fn test_serialize_i128_end_string_error() {
    struct MockWriter {
        should_fail: bool,
    }

    impl MockWriter {
        fn new(should_fail: bool) -> Self {
            MockWriter { should_fail }
        }
    }

    struct MockFormatter<'a> {
        writer: &'a mut MockWriter,
        should_fail_end: bool,
    }

    impl<'a> MockFormatter<'a> {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_i128(&mut self, _: &mut MockWriter, _: i128) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            if self.should_fail_end {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "end_string failed"))
            } else {
                Ok(())
            }
        }
    }

    struct MockSerializer<'a> {
        formatter: MockFormatter<'a>,
    }

    impl<'a> MockSerializer<'a> {
        fn new(writer: &'a mut MockWriter, should_fail_end: bool) -> Self {
            MockSerializer {
                formatter: MockFormatter { writer, should_fail_end },
            }
        }
    }

    struct TestSer {
        ser: MockSerializer<'static>,
    }

    impl TestSer {
        fn new(writer: &'static mut MockWriter, should_fail_end: bool) -> Self {
            TestSer {
                ser: MockSerializer::new(writer, should_fail_end),
            }
        }

        fn serialize_i128(self, value: i128) -> Result<()> {
            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(Error::io));
            tri!(self.ser.formatter.write_i128(&mut self.ser.writer, value).map_err(Error::io));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
        }
    }

    let mut writer = MockWriter::new(false); // should not fail on begin
    let test_ser = TestSer::new(&mut writer, true); // should fail on end
    let result = test_ser.serialize_i128(12345678901234567890i128);
    
    assert!(result.is_err());
}

