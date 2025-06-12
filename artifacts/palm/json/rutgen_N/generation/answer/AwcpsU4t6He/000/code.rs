// Answer 0

#[test]
fn test_begin_array_value_first() {
    use std::io::Cursor;
    use std::io::Result;

    struct Writer {
        cursor: Cursor<Vec<u8>>,
        current_indent: usize,
        indent: usize,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                cursor: Cursor::new(vec![]),
                current_indent: 0,
                indent: 4,
            }
        }

        fn begin_array_value(&mut self, first: bool) -> Result<()> {
            self.write_all(if first { b"\n" } else { b",\n" })?;
            indent(&mut self.cursor, self.current_indent, self.indent)
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.cursor.write_all(buf)
        }
    }

    fn indent<W: io::Write>(_writer: &mut W, _current_indent: usize, _indent: usize) -> Result<()> {
        Ok(())
    }

    let mut writer = Writer::new();
    let result = writer.begin_array_value(true);
    assert!(result.is_ok());
    assert_eq!(writer.cursor.get_ref().as_slice(), b"\n");
}

#[test]
fn test_begin_array_value_not_first() {
    use std::io::Cursor;
    use std::io::Result;

    struct Writer {
        cursor: Cursor<Vec<u8>>,
        current_indent: usize,
        indent: usize,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                cursor: Cursor::new(vec![]),
                current_indent: 0,
                indent: 4,
            }
        }

        fn begin_array_value(&mut self, first: bool) -> Result<()> {
            self.write_all(if first { b"\n" } else { b",\n" })?;
            indent(&mut self.cursor, self.current_indent, self.indent)
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.cursor.write_all(buf)
        }
    }

    fn indent<W: io::Write>(_writer: &mut W, _current_indent: usize, _indent: usize) -> Result<()> {
        Ok(())
    }

    let mut writer = Writer::new();
    let result = writer.begin_array_value(false);
    assert!(result.is_ok());
    assert_eq!(writer.cursor.get_ref().as_slice(), b",\n");
}

