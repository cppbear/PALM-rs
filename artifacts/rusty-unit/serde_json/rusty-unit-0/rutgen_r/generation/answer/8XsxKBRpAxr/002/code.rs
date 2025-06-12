// Answer 0

fn test_end_array_panic() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    struct MockWriter {
        should_return_err: bool,
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_return_err {
                return Err(io::Error::new(io::ErrorKind::Other, "write error"));
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        current_indent: usize,
        has_value: bool,
        indent: usize,
    }

    impl Serializer {
        fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                indent(writer, self.current_indent, self.indent)?;
            }

            writer.write_all(b"]")
        }
    }

    fn indent<W>(writer: &mut W, _current_indent: usize, _indent: usize) -> io::Result<()>
    where
        W: Write,
    {
        Err(io::Error::new(io::ErrorKind::Other, "indent error"))
    }

    let mut serializer = Serializer {
        current_indent: 1,
        has_value: true,
        indent: 2,
    };

    let mut mock_writer = MockWriter { should_return_err: true };

    let result = serializer.end_array(&mut mock_writer);
    assert!(result.is_err());

    Ok(())
}

