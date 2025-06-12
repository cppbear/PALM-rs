// Answer 0

#[test]
fn test_begin_object_key_writer_error() {
    use std::io;
    use std::io::Write;

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = FailingWriter;
    let mut ser = Ser { current_indent: 0, indent: 2 };

    let result = ser.begin_object_key(&mut writer, true);
    
    assert!(result.is_err());
} 

struct Ser {
    current_indent: usize,
    indent: usize,
}

impl Ser {
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(if first { b"\n" } else { b",\n" })?;
        indent(writer, self.current_indent, self.indent)
    }
}

fn indent<W>(writer: &mut W, current_indent: usize, indent: usize) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    // Simplified version of indent for test purposes
    writer.write_all(&vec![b' '; current_indent + indent])
}

