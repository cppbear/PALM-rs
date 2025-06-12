// Answer 0

#[derive(Debug)]
struct WriterMock {
    data: Vec<u8>,
}

impl WriterMock {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(bytes);
        Ok(bytes.len())
    }
}

#[derive(Debug)]
struct FormatterMock<'a> {
    writer: &'a mut WriterMock,
}

impl<'a> FormatterMock<'a> {
    fn begin_string(&mut self) -> Result<(), std::io::Error> {
        self.writer.write(b"{\"value\":").map(|_| ())
    }

    fn write_f64(&mut self, value: f64) -> Result<(), std::io::Error> {
        let bytes = value.to_string().as_bytes();
        self.writer.write(bytes).map(|_| ())
    }

    fn end_string(&mut self) -> Result<(), std::io::Error> {
        self.writer.write(b"}").map(|_| ())
    }
}

#[derive(Debug)]
struct Ser {
    formatter: FormatterMock<'static>,
}

#[derive(Debug)]
struct Serializer {
    ser: Ser,
}

impl Serializer {
    fn new(writer: &'static mut WriterMock) -> Self {
        Self {
            ser: Ser {
                formatter: FormatterMock { writer },
            },
        }
    }

    fn serialize_f64(self, value: f64) -> Result<(), String> {
        if !value.is_finite() {
            return Err("value must be finite".into());
        }
        self.ser.formatter.begin_string().map_err(|_| "IO error".into())?;
        self.ser.formatter.write_f64(value).map_err(|_| "IO error".into())?;
        self.ser.formatter.end_string().map_err(|_| "IO error".into())
    }
}

#[test]
fn test_serialize_f64_valid() {
    let mut writer = WriterMock::new();
    let serializer = Serializer::new(&mut writer);
    let result = serializer.serialize_f64(3.14);
    assert!(result.is_ok());
    assert_eq!(writer.data, b"{\"value\":3.14}");
}

#[test]
fn test_serialize_f64_negative() {
    let mut writer = WriterMock::new();
    let serializer = Serializer::new(&mut writer);
    let result = serializer.serialize_f64(-2.71);
    assert!(result.is_ok());
    assert_eq!(writer.data, b"{\"value\":-2.71}");
}

#[test]
fn test_serialize_f64_infinity() {
    let mut writer = WriterMock::new();
    let serializer = Serializer::new(&mut writer);
    let result = serializer.serialize_f64(f64::INFINITY);
    assert_eq!(result, Err("value must be finite".into()));
}

#[test]
fn test_serialize_f64_nan() {
    let mut writer = WriterMock::new();
    let serializer = Serializer::new(&mut writer);
    let result = serializer.serialize_f64(f64::NAN);
    assert_eq!(result, Err("value must be finite".into()));
}

