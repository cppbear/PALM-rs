// Answer 0

fn serialize_f32_test() {
    struct MockWriter;
    struct MockFormatter {
        should_begin_string_fail: bool,
        should_write_f32_fail: bool,
    }
    struct MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl MockWriter {
        fn write(&self, _data: &[u8]) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn begin_string(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            if self.should_begin_string_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string fail"))
            } else {
                Ok(())
            }
        }

        fn write_f32(&self, _writer: &mut MockWriter, _value: f32) -> Result<(), std::io::Error> {
            if self.should_write_f32_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write_f32 fail"))
            } else {
                Ok(())
            }
        }

        fn end_string(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    fn serialize_f32(ser: &mut MockSerializer, value: f32) -> Result<(), std::io::Error> {
        if !value.is_finite() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "value must be finite"));
        }

        ser.formatter.begin_string(&mut ser.writer)?;
        ser.formatter.write_f32(&mut ser.writer, value)?;
        ser.formatter.end_string(&mut ser.writer)?;

        Ok(())
    }

    // Test for a finite value
    let mut serializer = MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter {
            should_begin_string_fail: false,
            should_write_f32_fail: false,
        },
    };
    let result = serialize_f32(&mut serializer, 3.14);
    assert!(result.is_ok());

    // Test for a negative finite value
    let mut serializer_neg = MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter {
            should_begin_string_fail: false,
            should_write_f32_fail: false,
        },
    };
    let result_neg = serialize_f32(&mut serializer_neg, -1.0);
    assert!(result_neg.is_ok());

    // Test for NaN value
    let mut serializer_nan = MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter {
            should_begin_string_fail: false,
            should_write_f32_fail: false,
        },
    };
    let result_nan = serialize_f32(&mut serializer_nan, std::f32::NAN);
    assert!(result_nan.is_err());

    // Test for infinity value
    let mut serializer_inf = MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter {
            should_begin_string_fail: false,
            should_write_f32_fail: false,
        },
    };
    let result_inf = serialize_f32(&mut serializer_inf, std::f32::INFINITY);
    assert!(result_inf.is_err());

    // Test for failure in begin_string
    let mut serializer_fail_begin = MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter {
            should_begin_string_fail: true,
            should_write_f32_fail: false,
        },
    };
    let result_fail_begin = serialize_f32(&mut serializer_fail_begin, 2.5);
    assert!(result_fail_begin.is_err());

    // Test for failure in write_f32
    let mut serializer_fail_write = MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter {
            should_begin_string_fail: false,
            should_write_f32_fail: true,
        },
    };
    let result_fail_write = serialize_f32(&mut serializer_fail_write, 2.5);
    assert!(result_fail_write.is_err());
}

