// Answer 0

#[test]
fn test_serialize_f64_with_non_finite_value() {
    struct DummyWriter;
    impl std::io::Write for DummyWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Formatter<'a> {
        writer: &'a mut DummyWriter,
    }

    struct Serializer<'a> {
        formatter: Formatter<'a>,
    }

    struct TestSer {
        ser: Serializer<'static>,
    }

    impl TestSer {
        fn serialize_f64(self, value: f64) -> Result<(), String> {
            if !value.is_finite() {
                return Err("value must be finite".to_string());
            }

            self.ser.formatter.begin_string(&mut self.ser.writer).map_err(|_| "formatter_io_error".to_string())?;
            self.ser.formatter.write_f64(&mut self.ser.writer, value).map_err(|_| "formatter_io_error".to_string())?;
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(|_| "formatter_io_error".to_string())?;
            Ok(())
        }
    }

    let mut writer = DummyWriter;
    let formatter = Formatter { writer: &mut writer };
    let serializer = Serializer { formatter };
    let test_ser = TestSer { ser: serializer };

    let result = test_ser.serialize_f64(f64::INFINITY);
    assert_eq!(result, Err("value must be finite".to_string()));
}

#[test]
fn test_serialize_f64_with_io_error_on_begin_string() {
    struct FailingWriter;
    impl std::io::Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Formatter<'a> {
        writer: &'a mut FailingWriter,
    }

    struct Serializer<'a> {
        formatter: Formatter<'a>,
    }

    struct TestSer {
        ser: Serializer<'static>,
    }

    impl TestSer {
        fn serialize_f64(self, value: f64) -> Result<(), String> {
            if !value.is_finite() {
                return Err("value must be finite".to_string());
            }

            self.ser.formatter.begin_string(&mut self.ser.writer).map_err(|_| "formatter_io_error".to_string())?;
            self.ser.formatter.write_f64(&mut self.ser.writer, value).map_err(|_| "formatter_io_error".to_string())?;
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(|_| "formatter_io_error".to_string())?;
            Ok(())
        }
    }

    let mut writer = FailingWriter;
    let formatter = Formatter { writer: &mut writer };
    let serializer = Serializer { formatter };
    let test_ser = TestSer { ser: serializer };

    let result = test_ser.serialize_f64(1.0);
    assert_eq!(result, Err("formatter_io_error".to_string()));
}

