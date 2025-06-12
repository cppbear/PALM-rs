// Answer 0

#[test]
fn test_end_object_value() {
    use std::io::{self, Write};

    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { buffer: Vec::new() }
        }
    }

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestStruct {
        has_value: bool,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { has_value: false }
        }

        fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.has_value = true;
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let mut test_struct = TestStruct::new();
    let result = test_struct.end_object_value(&mut writer);
    
    assert!(result.is_ok());
    assert_eq!(test_struct.has_value, true);
}

