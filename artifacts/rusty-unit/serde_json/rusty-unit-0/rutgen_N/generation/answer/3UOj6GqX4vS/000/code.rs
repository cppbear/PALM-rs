// Answer 0

#[test]
fn test_end_array_value() {
    use std::io;

    struct TestWriter {
        data: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
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

        fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            self.has_value = true;
            Ok(())
        }
    }

    let mut writer = TestWriter { data: Vec::new() };
    let mut test_struct = TestStruct::new();
    
    assert!(!test_struct.has_value);
    let result = test_struct.end_array_value(&mut writer);
    assert!(result.is_ok());
    assert!(test_struct.has_value);
}

