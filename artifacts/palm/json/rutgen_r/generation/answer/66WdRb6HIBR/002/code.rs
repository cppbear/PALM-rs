// Answer 0

fn test_serialize_bool_success() {
    struct MockFormatter {
        call_begin: bool,
        call_write: bool,
        call_end: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                call_begin: true,
                call_write: true,
                call_end: true,
            }
        }

        fn begin_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            if self.call_begin {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Begin failed"))
            }
        }

        fn write_bool(&mut self, _: &mut dyn std::io::Write, _: bool) -> Result<(), std::io::Error> {
            if self.call_write {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Write failed"))
            }
        }

        fn end_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            if self.call_end {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "End failed"))
            }
        }
    }

    struct MockSerializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSerializer {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: MockFormatter::new(),
            }
        }
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl TestStruct {
        fn serialize_bool(self, value: bool) -> Result<()> {
            tri!(self
                .ser
                .formatter
                .begin_string(&mut self.ser.writer)
                .map_err(Error::io));
            tri!(self
                .ser
                .formatter
                .write_bool(&mut self.ser.writer, value)
                .map_err(Error::io));
            self.ser
                .formatter
                .end_string(&mut self.ser.writer)
                .map_err(Error::io)
        }
    }

    let serializer = MockSerializer::new();
    let test_struct = TestStruct { ser: serializer };

    let result = test_struct.serialize_bool(true);
    assert!(result.is_ok());
}

fn test_serialize_bool_begin_fail() {
    struct MockFormatter {
        call_begin: bool,
    }

    impl MockFormatter {
        fn new(call_begin: bool) -> Self {
            Self { call_begin }
        }

        fn begin_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            if self.call_begin {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Begin failed"))
            }
        }

        fn write_bool(&mut self, _: &mut dyn std::io::Write, _: bool) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSerializer {
        fn new(formatter: MockFormatter) -> Self {
            Self {
                writer: Vec::new(),
                formatter,
            }
        }
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl TestStruct {
        fn serialize_bool(self, value: bool) -> Result<()> {
            tri!(self
                .ser
                .formatter
                .begin_string(&mut self.ser.writer)
                .map_err(Error::io));
            tri!(self
                .ser
                .formatter
                .write_bool(&mut self.ser.writer, value)
                .map_err(Error::io));
            self.ser
                .formatter
                .end_string(&mut self.ser.writer)
                .map_err(Error::io)
        }
    }

    let formatter = MockFormatter::new(false);
    let serializer = MockSerializer::new(formatter);
    let test_struct = TestStruct { ser: serializer };

    let result = test_struct.serialize_bool(true);
    assert!(result.is_err());
}

fn test_serialize_bool_write_fail() {
    struct MockFormatter {
        call_write: bool,
    }

    impl MockFormatter {
        fn new(call_write: bool) -> Self {
            Self { call_write }
        }

        fn begin_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_bool(&mut self, _: &mut dyn std::io::Write, _: bool) -> Result<(), std::io::Error> {
            if self.call_write {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Write failed"))
            }
        }

        fn end_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSerializer {
        fn new(formatter: MockFormatter) -> Self {
            Self {
                writer: Vec::new(),
                formatter,
            }
        }
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl TestStruct {
        fn serialize_bool(self, value: bool) -> Result<()> {
            tri!(self
                .ser
                .formatter
                .begin_string(&mut self.ser.writer)
                .map_err(Error::io));
            tri!(self
                .ser
                .formatter
                .write_bool(&mut self.ser.writer, value)
                .map_err(Error::io));
            self.ser
                .formatter
                .end_string(&mut self.ser.writer)
                .map_err(Error::io)
        }
    }

    let formatter = MockFormatter::new(false);
    let serializer = MockSerializer::new(formatter);
    let test_struct = TestStruct { ser: serializer };

    let result = test_struct.serialize_bool(true);
    assert!(result.is_err());
}

fn test_serialize_bool_end_fail() {
    struct MockFormatter {
        call_end: bool,
    }

    impl MockFormatter {
        fn new(call_end: bool) -> Self {
            Self { call_end }
        }

        fn begin_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_bool(&mut self, _: &mut dyn std::io::Write, _: bool) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            if self.call_end {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "End failed"))
            }
        }
    }

    struct MockSerializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSerializer {
        fn new(formatter: MockFormatter) -> Self {
            Self {
                writer: Vec::new(),
                formatter,
            }
        }
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl TestStruct {
        fn serialize_bool(self, value: bool) -> Result<()> {
            tri!(self
                .ser
                .formatter
                .begin_string(&mut self.ser.writer)
                .map_err(Error::io));
            tri!(self
                .ser
                .formatter
                .write_bool(&mut self.ser.writer, value)
                .map_err(Error::io));
            self.ser
                .formatter
                .end_string(&mut self.ser.writer)
                .map_err(Error::io)
        }
    }

    let formatter = MockFormatter::new(false);
    let serializer = MockSerializer::new(formatter);
    let test_struct = TestStruct { ser: serializer };

    let result = test_struct.serialize_bool(true);
    assert!(result.is_err());
}

