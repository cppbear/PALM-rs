// Answer 0

#[test]
fn test_serialize_none_empty() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let serializer = &mut Serializer { writer };
    serializer.serialize_none();
}

#[test]
fn test_serialize_none_simple() {
    struct AnotherDummyWriter {
        data: Vec<u8>,
    }
    impl io::Write for AnotherDummyWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.data.extend_from_slice(buf);
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = AnotherDummyWriter { data: Vec::new() };
    let serializer = &mut Serializer { writer };
    serializer.serialize_none();
}

#[test]
fn test_serialize_none_with_state() {
    struct StateWriter;
    impl io::Write for StateWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(1)
        }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = StateWriter;
    let serializer = &mut Serializer { writer };
    serializer.serialize_none();
}

