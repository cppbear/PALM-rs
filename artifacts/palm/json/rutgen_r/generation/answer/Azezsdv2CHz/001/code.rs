// Answer 0

#[test]
fn test_end_object_write_all_error() {
    use std::io::{self, Write};

    struct TestWriter {
        should_fail: bool,
    }

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail {
                Err(io::Error::new(io::ErrorKind::Other, "test error"))
            } else {
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MyStruct {
        current_indent: usize,
        has_value: bool,
        indent: usize,
    }

    impl MyStruct {
        fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
            }

            writer.write_all(b"}")?;
            Ok(())
        }
    }

    let mut my_struct = MyStruct {
        current_indent: 1,
        has_value: true,
        indent: 0,
    };

    let mut writer = TestWriter { should_fail: true };
    
    let result = my_struct.end_object(&mut writer);
    assert!(result.is_err());
}

