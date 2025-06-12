// Answer 0

#[test]
fn test_serialize_u16_success() {
    struct Formatter {
        write_called: bool,
        begin_called: bool,
        end_called: bool,
    }

    struct Writer;

    struct Serializer {
        writer: Writer,
        formatter: Formatter,
    }

    impl Formatter {
        fn begin_string(&mut self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            self.begin_called = true;
            Ok(())
        }

        fn write_u16(&mut self, _writer: &mut Writer, _value: u16) -> Result<(), std::io::Error> {
            self.write_called = true;
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            self.end_called = true;
            Ok(())
        }
    }

    let mut formatter = Formatter {
        write_called: false,
        begin_called: false,
        end_called: false,
    };

    let serializer = Serializer {
        writer: Writer,
        formatter,
    };

    let result = serializer.serialize_u16(42);

    assert!(result.is_ok());
    assert!(serializer.formatter.begin_called);
    assert!(serializer.formatter.write_called);
    assert!(serializer.formatter.end_called);
}

#[test]
fn test_serialize_u16_failure_on_begin() {
    struct FailingFormatter {
        begin_succeeds: bool,
    }

    struct Writer;

    struct Serializer {
        writer: Writer,
        formatter: FailingFormatter,
    }

    impl FailingFormatter {
        fn begin_string(&mut self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            if self.begin_succeeds {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Beginning failed"))
            }
        }

        fn write_u16(&mut self, _writer: &mut Writer, _value: u16) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    let mut formatter = FailingFormatter { begin_succeeds: false };
    let serializer = Serializer {
        writer: Writer,
        formatter,
    };

    let result = serializer.serialize_u16(42);

    assert!(result.is_err());
}

#[test]
fn test_serialize_u16_failure_on_write() {
    struct FailingFormatter {
        write_succeeds: bool,
    }

    struct Writer;

    struct Serializer {
        writer: Writer,
        formatter: FailingFormatter,
    }

    impl FailingFormatter {
        fn begin_string(&mut self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_u16(&mut self, _writer: &mut Writer, _value: u16) -> Result<(), std::io::Error> {
            if self.write_succeeds {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Write failed"))
            }
        }

        fn end_string(&mut self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    let mut formatter = FailingFormatter { write_succeeds: false };
    let serializer = Serializer {
        writer: Writer,
        formatter,
    };

    let result = serializer.serialize_u16(42);

    assert!(result.is_err());
} 

#[test]
fn test_serialize_u16_failure_on_end() {
    struct FailingFormatter {
        end_succeeds: bool,
    }

    struct Writer;

    struct Serializer {
        writer: Writer,
        formatter: FailingFormatter,
    }
    
    impl FailingFormatter {
        fn begin_string(&mut self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_u16(&mut self, _writer: &mut Writer, _value: u16) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            if self.end_succeeds {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "End failed"))
            }
        }
    }

    let mut formatter = FailingFormatter { end_succeeds: false };
    let serializer = Serializer {
        writer: Writer,
        formatter,
    };

    let result = serializer.serialize_u16(42);

    assert!(result.is_err());
}

