// Answer 0

#[test]
fn test_serialize_f32_not_finite() {
    let mut writer = Vec::new();
    
    struct MockFormatter;
    
    impl MockFormatter {
        fn begin_string(&self, _: &mut Vec<u8>) -> Result<(), &'static str> {
            Err("Begin string error")
        }
        
        fn write_f32(&self, _: &mut Vec<u8>, _: f32) -> Result<(), &'static str> {
            Ok(())
        }
        
        fn end_string(&self, _: &mut Vec<u8>) -> Result<(), &'static str> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    struct MockSerializer {
        ser: MockSer,
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                ser: MockSer {
                    formatter: MockFormatter,
                    writer: Vec::new(),
                },
            }
        }
        
        fn serialize_f32(self, value: f32) -> Result<(), &'static str> {
            if !value.is_finite() {
                return Err("Value must be finite");
            }

            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_f32(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            
            Ok(())
        }
    }

    let serializer = MockSerializer::new();
    let result = serializer.serialize_f32(f32::INFINITY);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("Value must be finite"));
}

#[test]
fn test_serialize_f32_begin_string_error() {
    let mut writer = Vec::new();
    
    struct MockFormatter;
    
    impl MockFormatter {
        fn begin_string(&self, _: &mut Vec<u8>) -> Result<(), &'static str> {
            Err("Begin string error")
        }
        
        fn write_f32(&self, _: &mut Vec<u8>, _: f32) -> Result<(), &'static str> {
            Ok(())
        }
        
        fn end_string(&self, _: &mut Vec<u8>) -> Result<(), &'static str> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    struct MockSerializer {
        ser: MockSer,
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                ser: MockSer {
                    formatter: MockFormatter,
                    writer: Vec::new(),
                },
            }
        }
        
        fn serialize_f32(self, value: f32) -> Result<(), &'static str> {
            if !value.is_finite() {
                return Err("Value must be finite");
            }

            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_f32(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            
            Ok(())
        }
    }

    let serializer = MockSerializer::new();
    let result = serializer.serialize_f32(1.0);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("Begin string error"));
}

