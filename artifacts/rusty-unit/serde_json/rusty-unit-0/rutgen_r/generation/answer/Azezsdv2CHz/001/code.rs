// Answer 0

fn test_end_object_panic_on_write_all() {
    use std::io::{self, Write};

    struct TestWriter {
        should_fail: bool,
    }

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail {
                Err(io::Error::new(io::ErrorKind::Other, "write error"))
            } else {
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestStruct {
        current_indent: usize,
        indent: usize,
        has_value: bool,
    }

    impl TestStruct {
        fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                let _ = writer.write_all(b"\n");
                // Simulating the indent function directly
                for _ in 0..self.current_indent {
                    let _ = writer.write_all(b" ");
                }
            }

            writer.write_all(b"}")
        }
    }

    let mut test_struct = TestStruct {
        current_indent: 2,
        indent: 2,
        has_value: true,
    };

    let mut writer = TestWriter { should_fail: true };

    let result = test_struct.end_object(&mut writer);
    assert!(result.is_err());
}

