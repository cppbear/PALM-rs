// Answer 0

#[test]
fn test_serialize_f64_normal() {
    struct MockFormatter;
    impl MockFormatter {
        fn write_f64(&self, _: &mut MockWriter, _: f64) -> Result<()> {
            Ok(())
        }
        fn write_null(&self, _: &mut MockWriter) -> Result<()> {
            panic!("Should not call write_null");
        }
    }
    
    struct MockWriter;
    
    struct TestSerializer<W, F> {
        writer: W,
        formatter: F,
    }

    impl<W: MockWriter, F: MockFormatter> TestSerializer<W, F> {
        fn serialize_f64(self, value: f64) -> Result<()> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer).map_err(Error::io)
                }
                _ => {
                    self.formatter.write_f64(&mut self.writer, value).map_err(Error::io)
                }
            }
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = TestSerializer { writer, formatter };

    // Test with a finite value
    let result = serializer.serialize_f64(42.0);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Should not call write_null")]
fn test_serialize_f64_nan() {
    struct MockFormatter;
    impl MockFormatter {
        fn write_f64(&self, _: &mut MockWriter, _: f64) -> Result<()> {
            Ok(())
        }
        fn write_null(&self, _: &mut MockWriter) -> Result<()> {
            panic!("Should not call write_null");
        }
    }
    
    struct MockWriter;
    
    struct TestSerializer<W, F> {
        writer: W,
        formatter: F,
    }

    impl<W: MockWriter, F: MockFormatter> TestSerializer<W, F> {
        fn serialize_f64(self, value: f64) -> Result<()> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer).map_err(Error::io)
                }
                _ => {
                    self.formatter.write_f64(&mut self.writer, value).map_err(Error::io)
                }
            }
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = TestSerializer { writer, formatter };

    // Test with NaN value
    let _ = serializer.serialize_f64(f64::NAN);
}

#[test]
#[should_panic(expected = "Should not call write_null")]
fn test_serialize_f64_infinite() {
    struct MockFormatter;
    impl MockFormatter {
        fn write_f64(&self, _: &mut MockWriter, _: f64) -> Result<()> {
            Ok(())
        }
        fn write_null(&self, _: &mut MockWriter) -> Result<()> {
            panic!("Should not call write_null");
        }
    }
    
    struct MockWriter;
    
    struct TestSerializer<W, F> {
        writer: W,
        formatter: F,
    }

    impl<W: MockWriter, F: MockFormatter> TestSerializer<W, F> {
        fn serialize_f64(self, value: f64) -> Result<()> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer).map_err(Error::io)
                }
                _ => {
                    self.formatter.write_f64(&mut self.writer, value).map_err(Error::io)
                }
            }
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = TestSerializer { writer, formatter };

    // Test with infinite value
    let _ = serializer.serialize_f64(f64::INFINITY);
}

