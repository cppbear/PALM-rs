// Answer 0

#[derive(Default)]
struct Writer {
    buffer: Vec<u8>,
}

impl Writer {
    fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.buffer.extend_from_slice(data);
        Ok(())
    }
}

struct Formatter<'a> {
    writer: &'a mut Writer,
}

impl<'a> Formatter<'a> {
    fn begin_string(&mut self) -> Result<(), std::io::Error> {
        self.writer.write(b"\"")?;
        Ok(())
    }

    fn write_i16(&mut self, value: i16) -> Result<(), std::io::Error> {
        let bytes = value.to_string().as_bytes();
        self.writer.write(bytes)?;
        Ok(())
    }

    fn end_string(&mut self) -> Result<(), std::io::Error> {
        self.writer.write(b"\"")?;
        Ok(())
    }
}

struct Serializer<'a> {
    writer: Writer,
    formatter: Formatter<'a>,
}

impl<'a> Default for Serializer<'a> {
    fn default() -> Self {
        let mut writer = Writer::default();
        let formatter = Formatter { writer: &mut writer };
        Self { writer, formatter }
    }
}

fn serialize_i16(serializer: &mut Serializer, value: i16) -> Result<(), std::io::Error> {
    serializer.formatter.begin_string()?;
    serializer.formatter.write_i16(value)?;
    serializer.formatter.end_string()?;
    Ok(())
}

#[test]
fn test_serialize_i16() {
    let mut serializer = Serializer::default();
    let result = serialize_i16(&mut serializer, 12345);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer.buffer.clone()).unwrap(), "\"12345\"");
}

#[test]
fn test_serialize_i16_negative() {
    let mut serializer = Serializer::default();
    let result = serialize_i16(&mut serializer, -32768);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer.buffer.clone()).unwrap(), "\"-32768\"");
}

#[test]
fn test_serialize_i16_zero() {
    let mut serializer = Serializer::default();
    let result = serialize_i16(&mut serializer, 0);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer.buffer.clone()).unwrap(), "\"0\"");
}

