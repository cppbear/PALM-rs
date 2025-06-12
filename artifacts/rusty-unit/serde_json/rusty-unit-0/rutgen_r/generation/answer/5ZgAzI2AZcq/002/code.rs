// Answer 0

fn test_serialize_i16_success() {
    struct MockFormatter {
        should_fail: bool,
    }

    struct MockWriter;

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), String> {
            Ok(())
        }

        fn write_i16(&mut self, _: &mut MockWriter, _: i16) -> Result<(), String> {
            if self.should_fail {
                Err("Mock error".to_string())
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    struct Serializer {
        ser: MockSerializer,
    }

    impl Serializer {
        fn serialize_i16(self, value: i16) -> Result<(), String> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i16(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)
        }
    }

    let success_serializer = Serializer {
        ser: MockSerializer {
            writer: MockWriter,
            formatter: MockFormatter { should_fail: false },
        },
    };
    assert!(success_serializer.serialize_i16(123).is_ok());
}

fn test_serialize_i16_failure() {
    struct MockFormatter {
        should_fail: bool,
    }

    struct MockWriter;

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), String> {
            Ok(())
        }

        fn write_i16(&mut self, _: &mut MockWriter, _: i16) -> Result<(), String> {
            if self.should_fail {
                Err("Mock write error".to_string())
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    struct Serializer {
        ser: MockSerializer,
    }

    impl Serializer {
        fn serialize_i16(self, value: i16) -> Result<(), String> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i16(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)
        }
    }

    let failure_serializer = Serializer {
        ser: MockSerializer {
            writer: MockWriter,
            formatter: MockFormatter { should_fail: true },
        },
    };
    assert!(failure_serializer.serialize_i16(456).is_err());
}

