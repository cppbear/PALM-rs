// Answer 0

fn test_end_object_with_value() -> std::io::Result<()> {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    struct TestStruct {
        current_indent: usize,
        indent: usize,
        has_value: bool,
    }

    impl TestStruct {
        fn end_object<W>(&mut self, writer: &mut W) -> std::io::Result<()>
        where
            W: ?Sized + std::io::Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                self.indent(writer)?;
            }

            writer.write_all(b"}")
        }

        fn indent<W>(&self, writer: &mut W) -> std::io::Result<()>
        where
            W: ?Sized + std::io::Write,
        {
            // Simulate error condition for testing
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Indent error"))
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut test_struct = TestStruct {
        current_indent: 1,
        indent: 2,
        has_value: true,
    };

    let result = test_struct.end_object(&mut writer);
    assert!(result.is_err()); // expects Err
    assert_eq!(writer.buffer, b"\n"); // Ensures that the newline has been written

    Ok(())
}

#[test]
fn test_end_object_success() -> std::io::Result<()> {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    struct TestStruct {
        current_indent: usize,
        indent: usize,
        has_value: bool,
    }

    impl TestStruct {
        fn end_object<W>(&mut self, writer: &mut W) -> std::io::Result<()>
        where
            W: ?Sized + std::io::Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                self.indent(writer)?;
            }

            writer.write_all(b"}")
        }

        fn indent<W>(&self, writer: &mut W) -> std::io::Result<()>
        where
            W: ?Sized + std::io::Write,
        {
            // Now return successfully
            writer.write_all(b"  ") // Example indent
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut test_struct = TestStruct {
        current_indent: 1,
        indent: 2,
        has_value: true,
    };

    let result = test_struct.end_object(&mut writer);
    assert!(result.is_ok()); // expects Ok
    assert_eq!(writer.buffer, b"\n  }"); // Check the correct output

    Ok(())
}

