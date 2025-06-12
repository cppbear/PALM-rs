// Answer 0

#[test]
fn test_end_object_with_value() {
    use std::io::{self, Write};
    
    struct TestWriter {
        buffer: Vec<u8>,
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

    struct Serializer {
        current_indent: usize,
        has_value: bool,
        indent: usize,
    }

    impl Serializer {
        fn new(indent: usize) -> Self {
            Self {
                current_indent: 0,
                has_value: false,
                indent,
            }
        }

        fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                self.indent(writer, self.current_indent)?;
            }

            writer.write_all(b"}")?;
            Ok(())
        }

        fn indent<W>(&self, writer: &mut W, level: usize) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            writer.write_all(b"  ".repeat(level).as_slice())?;
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut serializer = Serializer {
        current_indent: 1,
        has_value: true,
        indent: 2,
    };

    let result = serializer.end_object(&mut writer);
    
    assert!(result.is_ok());
    assert_eq!(writer.buffer.as_slice(), b"\n  }");
}

#[test]
fn test_end_object_without_value() {
    use std::io::{self, Write};

    struct TestWriter {
        buffer: Vec<u8>,
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

    struct Serializer {
        current_indent: usize,
        has_value: bool,
        indent: usize,
    }

    impl Serializer {
        fn new(indent: usize) -> Self {
            Self {
                current_indent: 0,
                has_value: false,
                indent,
            }
        }

        fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                self.indent(writer, self.current_indent)?;
            }

            writer.write_all(b"}")?;
            Ok(())
        }

        fn indent<W>(&self, writer: &mut W, level: usize) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            writer.write_all(b"  ".repeat(level).as_slice())?;
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut serializer = Serializer {
        current_indent: 1,
        has_value: false,
        indent: 2,
    };

    let result = serializer.end_object(&mut writer);
    
    assert!(result.is_ok());
    assert_eq!(writer.buffer.as_slice(), b"}");
}

