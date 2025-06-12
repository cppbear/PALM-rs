// Answer 0

fn test_serialize_i64_success() {
    struct Formatter {
        should_fail: bool,
    }

    impl Formatter {
        fn begin_string(&self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string error"))
            } else {
                Ok(())
            }
        }

        fn write_i64(&self, _: &mut Vec<u8>, _: i64) -> Result<(), std::io::Error> {
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write_i64 error"))
            } else {
                Ok(())
            }
        }

        fn end_string(&self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "end_string error"))
            } else {
                Ok(())
            }
        }
    }

    struct Serializer {
        writer: Vec<u8>,
        formatter: Formatter,
    }

    impl Serializer {
        fn new(formatter: Formatter) -> Self {
            Self {
                writer: Vec::new(),
                formatter,
            }
        }

        fn serialize_i64(self, value: i64) -> Result<()> {
            tri!(self
                .formatter
                .begin_string(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i64(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string(&mut self.writer)
                .map_err(Error::io)
        }
    }

    // Test case for successful serialization
    let formatter = Formatter { should_fail: false };
    let serializer = Serializer::new(formatter);
    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i64_begin_string_failure() {
    struct Formatter {
        should_fail: bool,
    }

    impl Formatter {
        fn begin_string(&self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string error"))
            } else {
                Ok(())
            }
        }

        fn write_i64(&self, _: &mut Vec<u8>, _: i64) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        writer: Vec<u8>,
        formatter: Formatter,
    }

    impl Serializer {
        fn new(formatter: Formatter) -> Self {
            Self {
                writer: Vec::new(),
                formatter,
            }
        }

        fn serialize_i64(self, value: i64) -> Result<()> {
            tri!(self
                .formatter
                .begin_string(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i64(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string(&mut self.writer)
                .map_err(Error::io)
        }
    }

    // Test case where begin_string fails
    let formatter = Formatter { should_fail: true };
    let serializer = Serializer::new(formatter);
    let result = serializer.serialize_i64(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_i64_write_i64_failure() {
    struct Formatter {
        should_fail: bool,
    }

    impl Formatter {
        fn begin_string(&self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_i64(&self, _: &mut Vec<u8>, _: i64) -> Result<(), std::io::Error> {
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write_i64 error"))
            } else {
                Ok(())
            }
        }

        fn end_string(&self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        writer: Vec<u8>,
        formatter: Formatter,
    }

    impl Serializer {
        fn new(formatter: Formatter) -> Self {
            Self {
                writer: Vec::new(),
                formatter,
            }
        }

        fn serialize_i64(self, value: i64) -> Result<()> {
            tri!(self
                .formatter
                .begin_string(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i64(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string(&mut self.writer)
                .map_err(Error::io)
        }
    }

    // Test case where write_i64 fails
    let formatter = Formatter { should_fail: true };
    let serializer = Serializer::new(formatter);
    let result = serializer.serialize_i64(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_i64_end_string_failure() {
    struct Formatter {
        should_fail: bool,
    }

    impl Formatter {
        fn begin_string(&self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_i64(&self, _: &mut Vec<u8>, _: i64) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _: &mut Vec<u8>) -> Result<(), std::io::Error> {
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "end_string error"))
            } else {
                Ok(())
            }
        }
    }

    struct Serializer {
        writer: Vec<u8>,
        formatter: Formatter,
    }

    impl Serializer {
        fn new(formatter: Formatter) -> Self {
            Self {
                writer: Vec::new(),
                formatter,
            }
        }

        fn serialize_i64(self, value: i64) -> Result<()> {
            tri!(self
                .formatter
                .begin_string(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i64(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string(&mut self.writer)
                .map_err(Error::io)
        }
    }

    // Test case where end_string fails
    let formatter = Formatter { should_fail: true };
    let serializer = Serializer::new(formatter);
    let result = serializer.serialize_i64(42);
    assert!(result.is_err());
}

