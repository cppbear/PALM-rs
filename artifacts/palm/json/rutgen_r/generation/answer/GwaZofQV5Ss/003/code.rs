// Answer 0

#[test]
fn test_serialize_f64_valid() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            Ok(())
        }
        
        fn write_f64(&mut self, _writer: &mut Vec<u8>, _value: f64) -> Result<(), ()> {
            Ok(())
        }
        
        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    struct MockSelf {
        ser: MockSer,
    }

    impl MockSelf {
        fn serialize_f64(self, value: f64) -> Result<(), ()> {
            if !value.is_finite() {
                return Err(());
            }

            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_f64(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;

            Ok(())
        }
    }

    let mock_self = MockSelf {
        ser: MockSer {
            formatter: MockFormatter,
            writer: vec![],
        },
    };

    let result = mock_self.serialize_f64(3.14);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_f64_non_finite() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            Ok(())
        }
        
        fn write_f64(&mut self, _writer: &mut Vec<u8>, _value: f64) -> Result<(), ()> {
            Ok(())
        }
        
        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    struct MockSelf {
        ser: MockSer,
    }

    impl MockSelf {
        fn serialize_f64(self, value: f64) -> Result<(), ()> {
            if !value.is_finite() {
                return Err(());
            }

            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_f64(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;

            Ok(())
        }
    }

    let mock_self = MockSelf {
        ser: MockSer {
            formatter: MockFormatter,
            writer: vec![],
        },
    };

    let _ = mock_self.serialize_f64(f64::INFINITY);
}

