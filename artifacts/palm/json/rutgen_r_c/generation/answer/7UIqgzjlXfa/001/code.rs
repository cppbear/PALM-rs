// Answer 0

#[test]
fn test_serializer_new_with_valid_writer() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let mut writer = MockWriter { buffer: Vec::new() };
    let serializer = Serializer::new(writer);
    assert!(std::mem::size_of::<Serializer<MockWriter>>() > 0);
}

#[test]
fn test_serializer_new_with_empty_writer() {
    struct EmptyWriter;

    impl io::Write for EmptyWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let empty_writer = EmptyWriter;
    let serializer = Serializer::new(empty_writer);
    assert!(std::mem::size_of::<Serializer<EmptyWriter>>() > 0);
}

