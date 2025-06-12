// Answer 0

fn test_serialize_f64_valid() -> Result<()> {
    struct MockWriter;
    struct MockFormatter {
        begin_called: bool,
        write_called: bool,
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    struct Serializer {
        ser: MockSer,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<()> {
            self.begin_called = true;
            Ok(())
        }

        fn write_f64(&mut self, _: &mut MockWriter, _: f64) -> Result<()> {
            self.write_called = true;
            Ok(())
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    impl MockSer {
        fn new() -> Self {
            MockSer {
                formatter: MockFormatter {
                    begin_called: false,
                    write_called: false,
                },
                writer: MockWriter,
            }
        }
    }

    let mut serializer = Serializer {
        ser: MockSer::new(),
    };

    let result = serializer.serialize_f64(42.0);
    assert!(result.is_ok());
    assert!(serializer.ser.formatter.begin_called);
    assert!(serializer.ser.formatter.write_called);
    Ok(())
}

#[test]
fn test_serialize_f64_non_finite() {
    struct MockWriter;
    struct MockFormatter;

    struct MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    struct Serializer {
        ser: MockSer,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, _: &mut MockWriter, _: f64) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    impl MockSer {
        fn new() -> Self {
            MockSer {
                formatter: MockFormatter,
                writer: MockWriter,
            }
        }
    }

    let mut serializer = Serializer {
        ser: MockSer::new(),
    };

    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_err());
}

#[test]
fn test_serialize_f64_io_error_begin() {
    struct MockWriter;
    struct MockFormatter;

    struct MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    struct Serializer {
        ser: MockSer,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Err(Error::io)
        }

        fn write_f64(&mut self, _: &mut MockWriter, _: f64) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    impl MockSer {
        fn new() -> Self {
            MockSer {
                formatter: MockFormatter,
                writer: MockWriter,
            }
        }
    }

    let mut serializer = Serializer {
        ser: MockSer::new(),
    };

    let result = serializer.serialize_f64(42.0);
    assert!(result.is_err());
}

#[test]
fn test_serialize_f64_io_error_write() {
    struct MockWriter;
    struct MockFormatter {
        write_failure: bool,
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    struct Serializer {
        ser: MockSer,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, _: &mut MockWriter, _: f64) -> Result<()> {
            if self.write_failure {
                Err(Error::io)
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    impl MockSer {
        fn new() -> Self {
            MockSer {
                formatter: MockFormatter { write_failure: true },
                writer: MockWriter,
            }
        }
    }

    let mut serializer = Serializer {
        ser: MockSer::new(),
    };

    let result = serializer.serialize_f64(42.0);
    assert!(result.is_err());
}

