// Answer 0

fn end_array_test() -> std::io::Result<()> {
    use std::io::Cursor;
    use std::io::{self, Write};

    struct MyStruct {
        current_indent: usize,
        has_value: bool,
        indent: usize,
    }

    impl MyStruct {
        fn new() -> Self {
            MyStruct {
                current_indent: 1,
                has_value: true,
                indent: 4,
            }
        }

        fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + Write,
        {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                indent(writer, self.current_indent, self.indent)?;
            }

            writer.write_all(b"]")
        }
    }

    fn indent<W>(writer: &mut W, current_indent: usize, indent: usize) -> io::Result<()>
    where
        W: Write,
    {
        writer.write_all(b" ".repeat(current_indent * indent).as_slice())
    }

    let mut buf = Cursor::new(Vec::new());
    let mut my_struct = MyStruct::new();

    my_struct.end_array(&mut buf)?;

    assert_eq!(buf.get_ref(), b"\n    ]");
    Ok(())
}

#[test]
fn test_end_array() {
    end_array_test().expect("Test failed");
}

