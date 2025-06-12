// Answer 0

#[test]
fn test_serialize_f64_nan() {
    struct TestFormatter;
    
    impl TestFormatter {
        fn write_null(&mut self, _: &mut dyn io::Write) -> Result<()> {
            // Simulate writing null
            Ok(())
        }
        
        fn write_f64(&mut self, _: &mut dyn io::Write, _: f64) -> Result<()> {
            panic!("should not be called");
        }
    }
    
    struct TestWriter;
    
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        
        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = &mut Serializer { writer, formatter };

    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f64_infinite() {
    struct TestFormatter;
    
    impl TestFormatter {
        fn write_null(&mut self, _: &mut dyn io::Write) -> Result<()> {
            // Simulate writing null
            Ok(())
        }
        
        fn write_f64(&mut self, _: &mut dyn io::Write, _: f64) -> Result<()> {
            panic!("should not be called");
        }
    }
    
    struct TestWriter;
    
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        
        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = &mut Serializer { writer, formatter };

    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f64_normal() {
    struct TestFormatter;
    
    impl TestFormatter {
        fn write_null(&mut self, _: &mut dyn io::Write) -> Result<()> {
            panic!("should not be called");
        }
        
        fn write_f64(&mut self, _: &mut dyn io::Write, value: f64) -> Result<()> {
            assert_eq!(value, 3.14);
            Ok(())
        }
    }
    
    struct TestWriter;
    
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        
        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = &mut Serializer { writer, formatter };

    let result = serializer.serialize_f64(3.14);
    assert!(result.is_ok());
}

