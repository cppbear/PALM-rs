// Answer 0

#[test]
fn test_serialize_f32_normal_value() {
    struct FormatterMock;
    
    impl FormatterMock {
        fn write_f32(&self, writer: &mut dyn std::io::Write, value: f32) -> std::io::Result<()> {
            write!(writer, "{}", value)
        }
        
        fn write_null(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(writer, "null")
        }
    }
    
    struct Serializer {
        formatter: FormatterMock,
        writer: Vec<u8>,
    }
    
    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: FormatterMock,
                writer: Vec::new(),
            }
        }
        
        fn serialize_f32(self, value: f32) -> Result<()> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f32(&mut self.writer, value)
                    .map_err(Error::io),
            }
        }
    }
    
    let serializer = Serializer::new();
    let result = serializer.serialize_f32(3.14);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "3.14");
}

#[test]
fn test_serialize_f32_zero() {
    struct FormatterMock;
    
    impl FormatterMock {
        fn write_f32(&self, writer: &mut dyn std::io::Write, value: f32) -> std::io::Result<()> {
            write!(writer, "{}", value)
        }
        
        fn write_null(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(writer, "null")
        }
    }
    
    struct Serializer {
        formatter: FormatterMock,
        writer: Vec<u8>,
    }
    
    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: FormatterMock,
                writer: Vec::new(),
            }
        }
        
        fn serialize_f32(self, value: f32) -> Result<()> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f32(&mut self.writer, value)
                    .map_err(Error::io),
            }
        }
    }
    
    let serializer = Serializer::new();
    let result = serializer.serialize_f32(0.0);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "0");
}

#[test]
fn test_serialize_f32_negative_value() {
    struct FormatterMock;
    
    impl FormatterMock {
        fn write_f32(&self, writer: &mut dyn std::io::Write, value: f32) -> std::io::Result<()> {
            write!(writer, "{}", value)
        }
        
        fn write_null(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(writer, "null")
        }
    }
    
    struct Serializer {
        formatter: FormatterMock,
        writer: Vec<u8>,
    }
    
    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: FormatterMock,
                writer: Vec::new(),
            }
        }
        
        fn serialize_f32(self, value: f32) -> Result<()> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f32(&mut self.writer, value)
                    .map_err(Error::io),
            }
        }
    }
    
    let serializer = Serializer::new();
    let result = serializer.serialize_f32(-2.5);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "-2.5");
}

#[test]
fn test_serialize_f32_small_positive_value() {
    struct FormatterMock;
    
    impl FormatterMock {
        fn write_f32(&self, writer: &mut dyn std::io::Write, value: f32) -> std::io::Result<()> {
            write!(writer, "{}", value)
        }
        
        fn write_null(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(writer, "null")
        }
    }
    
    struct Serializer {
        formatter: FormatterMock,
        writer: Vec<u8>,
    }
    
    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: FormatterMock,
                writer: Vec::new(),
            }
        }
        
        fn serialize_f32(self, value: f32) -> Result<()> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f32(&mut self.writer, value)
                    .map_err(Error::io),
            }
        }
    }
    
    let serializer = Serializer::new();
    let result = serializer.serialize_f32(1e-10);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "1e-10");
}

