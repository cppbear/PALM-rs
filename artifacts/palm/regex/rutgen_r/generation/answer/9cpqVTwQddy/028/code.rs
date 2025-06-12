// Answer 0

#[derive(Debug)]
struct MockWriter {
    output: String,
}

impl MockWriter {
    fn new() -> Self {
        MockWriter { output: String::new() }
    }

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

mod ast {
    pub struct ClassAscii {
        pub kind: ClassAsciiKind,
        pub negated: bool,
    }

    #[derive(Debug)]
    pub enum ClassAsciiKind {
        Alnum,
        // Other variants are omitted for brevity.
    }
}

impl std::fmt::Write for MockWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.write_str(s)
    }
}

fn fmt_class_ascii(wtr: &mut MockWriter, ast: &ast::ClassAscii) -> std::fmt::Result {
    use ast::ClassAsciiKind::*;
    match ast.kind {
        ClassAsciiKind::Alnum if ast.negated => wtr.write_str("[:^alnum:]"),
        ClassAsciiKind::Alnum => wtr.write_str("[:alnum:]"),
        // Other match arms are omitted for brevity.
    }
}

#[test]
fn test_fmt_class_ascii_alnum() {
    let mut writer = MockWriter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    };
    fmt_class_ascii(&mut writer, &ast).unwrap();
    assert_eq!(writer.output, "[:alnum:]");
}

#[test]
fn test_fmt_class_ascii_alnum_negated() {
    let mut writer = MockWriter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };
    fmt_class_ascii(&mut writer, &ast).unwrap();
    assert_eq!(writer.output, "[:^alnum:]");
}

