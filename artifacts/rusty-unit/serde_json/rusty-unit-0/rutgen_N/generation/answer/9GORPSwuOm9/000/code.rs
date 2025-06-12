// Answer 0

#[test]
fn test_begin_object_key_first() {
    use std::io::Cursor;
    use std::io::{self, Write};

    struct TestWriter {
        buf: Vec<u8>,
        current_indent: usize,
        indent: usize,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                buf: Vec::new(),
                current_indent: 0,
                indent: 2,
            }
        }

        fn begin_object_key(&mut self, writer: &mut dyn Write, first: bool) -> io::Result<()> {
            writer.write_all(if first { b"\n" } else { b",\n" })?;
            for _ in 0..self.current_indent {
                writer.write_all(b" ")?;
            }
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let mut cursor = Cursor::new(&mut writer.buf);
    writer.begin_object_key(&mut cursor, true).unwrap();
    
    assert_eq!(writer.buf, b"\n");
}

#[test]
fn test_begin_object_key_not_first() {
    use std::io::Cursor;
    use std::io::{self, Write};

    struct TestWriter {
        buf: Vec<u8>,
        current_indent: usize,
        indent: usize,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                buf: Vec::new(),
                current_indent: 0,
                indent: 2,
            }
        }

        fn begin_object_key(&mut self, writer: &mut dyn Write, first: bool) -> io::Result<()> {
            writer.write_all(if first { b"\n" } else { b",\n" })?;
            for _ in 0..self.current_indent {
                writer.write_all(b" ")?;
            }
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let mut cursor = Cursor::new(&mut writer.buf);
    writer.begin_object_key(&mut cursor, false).unwrap();
    
    assert_eq!(writer.buf, b",\n");
}

