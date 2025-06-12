// Answer 0

fn test_end_array_no_value() -> std::io::Result<()> {
    use std::io::{Cursor, Write};

    struct Serializer {
        current_indent: usize,
        has_value: bool,
        indent: usize,
    }

    impl Serializer {
        fn end_array<W>(&mut self, writer: &mut W) -> std::io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                self.indent(writer, self.current_indent, self.indent)?;
            }

            writer.write_all(b"]")
        }

        fn indent<W>(&self, writer: &mut W, current_indent: usize, indent: usize) -> std::io::Result<()>
        where
            W: ?Sized + Write,
        {
            // Example indent logic (could be more complex)
            let spaces = " ".repeat(current_indent * indent);
            writer.write_all(spaces.as_bytes())
        }
    }

    let mut buf = Cursor::new(Vec::new());
    let mut serializer = Serializer {
        current_indent: 1,
        has_value: false,
        indent: 4,
    };

    serializer.end_array(&mut buf)?;

    let result = buf.into_inner();
    assert_eq!(&result, b"]");
    Ok(())
}

#[test]
fn test_end_array_no_value_success() {
    test_end_array_no_value().unwrap();
}

