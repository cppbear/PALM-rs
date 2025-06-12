// Answer 0

#[test]
fn test_end_object_with_no_value() {
    use std::io::Cursor;
    use std::io::Write;
    use std::io;

    struct TestWriter {
        current_indent: usize,
        has_value: bool,
        indent: usize,
    }

    impl TestWriter {
        fn new(has_value: bool) -> Self {
            TestWriter {
                current_indent: 0,
                has_value,
                indent: 2,
            }
        }

        fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                indent(writer, self.current_indent, self.indent)?;
            }

            writer.write_all(b"}")
        }
    }

    fn indent<W>(writer: &mut W, current_indent: usize, indent: usize) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        for _ in 0..current_indent * indent {
            writer.write_all(b" ")?;
        }
        Ok(())
    }

    let mut buffer = Cursor::new(Vec::new());
    let mut writer = TestWriter::new(false);

    let result = writer.end_object(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer.get_ref().as_slice(), b"}");
}

