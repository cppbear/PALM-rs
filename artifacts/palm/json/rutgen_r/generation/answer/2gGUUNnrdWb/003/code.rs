// Answer 0

fn test_serialize_u32_success() {
    struct TestFormatter {
        pub write_called: bool,
        pub begin_called: bool,
    }

    struct TestWriter;

    impl TestFormatter {
        fn begin_string(&mut self, _: &mut TestWriter) -> Result<(), Error> {
            self.begin_called = true;
            Ok(())
        }

        fn write_u32(&mut self, _: &mut TestWriter, _: u32) -> Result<(), Error> {
            self.write_called = true;
            Ok(())
        }

        fn end_string(&mut self, _: &mut TestWriter) -> Result<(), Error> {
            Ok(())
        }
    }

    struct TestSer {
        pub formatter: TestFormatter,
        pub writer: TestWriter,
    }

    struct TestStruct {
        pub ser: TestSer,
    }

    let mut formatter = TestFormatter {
        write_called: false,
        begin_called: false,
    };
    let writer = TestWriter;
    let ser = TestSer {
        formatter,
        writer,
    };
    let test_struct = TestStruct { ser };

    let result = test_struct.serialize_u32(42);
    assert!(result.is_ok());
    assert!(test_struct.ser.formatter.begin_called);
    assert!(test_struct.ser.formatter.write_called);
}

fn test_serialize_u32_failure_begin_string() {
    struct TestFormatter;

    struct TestWriter;

    impl TestFormatter {
        fn begin_string(&mut self, _: &mut TestWriter) -> Result<(), Error> {
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "Begin error")))
        }

        fn write_u32(&mut self, _: &mut TestWriter, _: u32) -> Result<(), Error> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut TestWriter) -> Result<(), Error> {
            Ok(())
        }
    }

    struct TestSer {
        pub formatter: TestFormatter,
        pub writer: TestWriter,
    }

    struct TestStruct {
        pub ser: TestSer,
    }

    let formatter = TestFormatter;
    let writer = TestWriter;
    let ser = TestSer { formatter, writer };
    let test_struct = TestStruct { ser };

    let result = test_struct.serialize_u32(42);
    assert!(result.is_err());
}

fn test_serialize_u32_failure_write_u32() {
    struct TestFormatter {
        pub write_should_fail: bool,
    }

    struct TestWriter;

    impl TestFormatter {
        fn begin_string(&mut self, _: &mut TestWriter) -> Result<(), Error> {
            Ok(())
        }

        fn write_u32(&mut self, _: &mut TestWriter, _: u32) -> Result<(), Error> {
            if self.write_should_fail {
                Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "Write error")))
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _: &mut TestWriter) -> Result<(), Error> {
            Ok(())
        }
    }

    struct TestSer {
        pub formatter: TestFormatter,
        pub writer: TestWriter,
    }

    struct TestStruct {
        pub ser: TestSer,
    }

    let formatter = TestFormatter {
        write_should_fail: true,
    };
    let writer = TestWriter;
    let ser = TestSer { formatter, writer };
    let test_struct = TestStruct { ser };

    let result = test_struct.serialize_u32(42);
    assert!(result.is_err());
}

fn test_serialize_u32_failure_end_string() {
    struct TestFormatter {
        pub end_should_fail: bool,
    }

    struct TestWriter;

    impl TestFormatter {
        fn begin_string(&mut self, _: &mut TestWriter) -> Result<(), Error> {
            Ok(())
        }

        fn write_u32(&mut self, _: &mut TestWriter, _: u32) -> Result<(), Error> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut TestWriter) -> Result<(), Error> {
            if self.end_should_fail {
                Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "End error")))
            } else {
                Ok(())
            }
        }
    }

    struct TestSer {
        pub formatter: TestFormatter,
        pub writer: TestWriter,
    }

    struct TestStruct {
        pub ser: TestSer,
    }

    let formatter = TestFormatter {
        end_should_fail: true,
    };
    let writer = TestWriter;
    let ser = TestSer { formatter, writer };
    let test_struct = TestStruct { ser };

    let result = test_struct.serialize_u32(42);
    assert!(result.is_err());
}

