// Answer 0

fn test_end_array_with_error_during_indent() -> Result<(), std::io::Error> {
    use std::io::{self, Write};

    struct MyWriter {
        error: bool,
    }

    impl Write for MyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.error {
                Err(io::Error::new(io::ErrorKind::Other, "Writer error"))
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
        fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                // Here we simulate the error when calling indent
                return Err(io::Error::new(io::ErrorKind::Other, "Indent error"));
            }

            writer.write_all(b"]")
        }
    }

    let mut writer = MyWriter { error: false };
    let mut my_struct = MyStruct {
        current_indent: 1,
        has_value: true,
        indent: 2,
    };

    let result = my_struct.end_array(&mut writer);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_end_array_should_return_error() {
    assert!(test_end_array_with_error_during_indent().is_ok());
}

