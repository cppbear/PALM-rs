// Answer 0

fn serialize_i8(self: &mut TestSerializer, value: i8) -> Result<()> {
    tri!(self
        .ser
        .formatter
        .begin_string(&mut self.ser.writer)
        .map_err(Error::io));
    tri!(self
        .ser
        .formatter
        .write_i8(&mut self.ser.writer, value)
        .map_err(Error::io));
    self.ser
        .formatter
        .end_string(&mut self.ser.writer)
        .map_err(Error::io)
}

#[derive(Debug)]
struct WriterMock {
    success: bool,
}

impl WriterMock {
    fn new(success: bool) -> Self {
        Self { success }
    }
}

impl Write for WriterMock {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.success {
            Ok(buf.len())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write failed"))
        }
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
struct FormatterMock<'a> {
    writer: &'a mut WriterMock,
}

impl<'a> FormatterMock<'a> {
    fn begin_string(&mut self, writer: &'a mut WriterMock) -> Result<()> {
        writer.write(b"\"")?;
        Ok(())
    }
    
    fn write_i8(&mut self, writer: &'a mut WriterMock, value: i8) -> Result<()> {
        if value == i8::MIN {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write failed").into())
        } else {
            writer.write(value.to_string().as_bytes())?;
            Ok(())
        }
    }
    
    fn end_string(&mut self, writer: &'a mut WriterMock) -> Result<()> {
        writer.write(b"\"")?;
        Ok(())
    }
}

#[derive(Debug)]
struct Serializer<'a> {
    writer: WriterMock,
    formatter: FormatterMock<'a>,
}

struct TestSerializer {
    ser: Serializer<'static>,
}

#[test]
fn test_serialize_i8_success() {
    let mut writer = WriterMock::new(true);
    let formatter = FormatterMock { writer: &mut writer };
    let ser = Serializer { writer, formatter };
    let mut test_serializer = TestSerializer { ser };

    assert!(serialize_i8(&mut test_serializer, 1).is_ok());
}

#[test]
fn test_serialize_i8_error_on_write() {
    let mut writer = WriterMock::new(false);
    let formatter = FormatterMock { writer: &mut writer };
    let ser = Serializer { writer, formatter };
    let mut test_serializer = TestSerializer { ser };

    assert!(serialize_i8(&mut test_serializer, i8::MIN).is_err());
}

