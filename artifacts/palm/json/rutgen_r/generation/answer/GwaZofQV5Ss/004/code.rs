// Answer 0

#[test]
fn test_serialize_f64_non_finite() {
    struct MockFormatter {
        called_begin_string: bool,
        called_write_f64: bool,
        called_end_string: bool,
    }

    struct MockWriter;

    struct MockSerializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl MockFormatter {
        fn begin_string(&self, _writer: &mut MockWriter) -> Result<(), ()> {
            Ok(())
        }

        fn write_f64(&self, _writer: &mut MockWriter, _value: f64) -> Result<(), ()> {
            Ok(())
        }

        fn end_string(&self, _writer: &mut MockWriter) -> Result<(), ()> {
            Ok(())
        }
    }

    impl MockSerializer {
        fn serialize_f64(self, value: f64) -> Result<(), ()> {
            if !value.is_finite() {
                return Err(());
            }

            self.formatter
                .begin_string(&mut self.writer)?;
            self.formatter
                .write_f64(&mut self.writer, value)?;
            self.formatter
                .end_string(&mut self.writer)?;

            Ok(())
        }
    }

    let serializer = MockSerializer {
        formatter: MockFormatter {
            called_begin_string: false,
            called_write_f64: false,
            called_end_string: false,
        },
        writer: MockWriter,
    };

    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_err());

    let result_infinity = serializer.serialize_f64(f64::INFINITY);
    assert!(result_infinity.is_err());

    let result_neg_infinity = serializer.serialize_f64(f64::NEG_INFINITY);
    assert!(result_neg_infinity.is_err());
}

