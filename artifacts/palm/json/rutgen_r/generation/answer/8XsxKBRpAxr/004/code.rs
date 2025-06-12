// Answer 0

#[test]
fn test_end_array_without_value() {
    use std::io::{self, Write};

    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { buffer: Vec::new() }
        }

        fn get_output(&self) -> String {
            String::from_utf8_lossy(&self.buffer).to_string()
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

    struct Serializer {
        current_indent: usize,
        has_value: bool,
        indent: usize,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                current_indent: 0,
                has_value: false,
                indent: 4, // assuming a common indent spacing
            }
        }

        fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                self.indent(writer)?;
            }

            writer.write_all(b"]")
        }

        fn indent<W>(&self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            for _ in 0..self.current_indent {
                writer.write_all(b" ")?; // simple space indentation
            }
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let mut serializer = Serializer::new();
    
    // Set current_indent to 1 to simulate nesting.
    serializer.current_indent = 1;

    // Test the end_array method with has_value set to false
    let result = serializer.end_array(&mut writer);
    assert!(result.is_ok());
    assert_eq!(writer.get_output(), b"]".to_vec());
}

