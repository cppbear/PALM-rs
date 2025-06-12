// Answer 0

#[test]
fn test_serialize_unit_struct_empty_name() {
    let writer = MockWriter::new();
    let serializer = Serializer { writer };
    serializer.serialize_unit_struct("")
}

#[test]
fn test_serialize_unit_struct_one_character_name() {
    let writer = MockWriter::new();
    let serializer = Serializer { writer };
    serializer.serialize_unit_struct("A")
}

#[test]
fn test_serialize_unit_struct_two_character_name() {
    let writer = MockWriter::new();
    let serializer = Serializer { writer };
    serializer.serialize_unit_struct("AB")
}

#[test]
fn test_serialize_unit_struct_full_alphabet() {
    let writer = MockWriter::new();
    let serializer = Serializer { writer };
    serializer.serialize_unit_struct("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
}

#[test]
fn test_serialize_unit_struct_long_string() {
    let long_name = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".repeat(39); // 1024 characters
    let writer = MockWriter::new();
    let serializer = Serializer { writer };
    serializer.serialize_unit_struct(&long_name)
}

struct MockWriter;

impl io::Write for MockWriter {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        Ok(_buf.len())
    }
    fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
        Ok(())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

