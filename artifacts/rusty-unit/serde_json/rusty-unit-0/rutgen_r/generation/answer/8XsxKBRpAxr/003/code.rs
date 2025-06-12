// Answer 0

fn end_array_test() -> io::Result<()> {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for TestWriter {
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
        fn new(current_indent: usize, has_value: bool, indent: usize) -> Self {
            Serializer {
                current_indent,
                has_value,
                indent,
            }
        }

        fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                self.indent(writer, self.current_indent, self.indent)?;
            }

            writer.write_all(b"]")
        }

        fn indent<W>(&self, writer: &mut W, current_indent: usize, indent: usize) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            let indentation: Vec<u8> = vec![b' '; current_indent * indent];
            writer.write_all(&indentation)
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut serializer = Serializer::new(2, true, 4);
    
    let result = serializer.end_array(&mut writer)?;

    assert_eq!(result, Ok(()));
    assert_eq!(writer.buffer, b"\n    ]"); // Expecting the new line and the closing bracket with indent

    Ok(())
}

#[test]
fn test_end_array() {
    end_array_test().unwrap();
}

