// Answer 0

fn test_begin_object_key_first_false() -> std::io::Result<()> {
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
        indent: usize,
    }

    impl Serializer {
        fn new(current_indent: usize, indent: usize) -> Self {
            Serializer { current_indent, indent }
        }

        fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            writer.write_all(if first { b"\n" } else { b",\n" })?;
            self.indent(writer, self.current_indent, self.indent)
        }

        fn indent<W>(&self, writer: &mut W, current_indent: usize, indent: usize) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            let spaces = " ".repeat(current_indent + indent);
            writer.write_all(spaces.as_bytes())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut serializer = Serializer::new(2, 2);

    // Test with the first parameter as false.
    serializer.begin_object_key(&mut writer, false)?;

    // Assert the output is as expected.
    let expected_output = b",\n    "; // Since current_indent and indent are both 2
    assert_eq!(writer.buffer, expected_output);

    Ok(())
}

#[test]
fn test_successful_begin_object_key() {
    test_begin_object_key_first_false().unwrap();
}

