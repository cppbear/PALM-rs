// Answer 0

#[test]
fn test_serialize_f64_non_finite() {
    struct MockFormatter {
        writer: Vec<u8>,
    }

    struct MockSerializer {
        formatter: MockFormatter,
    }

    impl MockFormatter {
        fn begin_string(&mut self) -> Result<(), ()> {
            Ok(())
        }

        fn write_f64(&mut self, _value: f64) -> Result<(), ()> {
            Ok(())
        }

        fn end_string(&mut self) -> Result<(), ()> {
            Ok(())
        }
    }

    impl MockSerializer {
        fn serialize_f64(&mut self, value: f64) -> Result<(), ()> {
            if !value.is_finite() {
                return Err(());
            }

            self.formatter.begin_string()?;
            self.formatter.write_f64(value)?;
            self.formatter.end_string()
        }
    }

    let mut serializer = MockSerializer {
        formatter: MockFormatter { writer: Vec::new() },
    };

    let result_infinity = serializer.serialize_f64(f64::INFINITY);
    assert!(result_infinity.is_err());

    let result_negative_infinity = serializer.serialize_f64(f64::NEG_INFINITY);
    assert!(result_negative_infinity.is_err());

    let result_nan = serializer.serialize_f64(f64::NAN);
    assert!(result_nan.is_err());
}

