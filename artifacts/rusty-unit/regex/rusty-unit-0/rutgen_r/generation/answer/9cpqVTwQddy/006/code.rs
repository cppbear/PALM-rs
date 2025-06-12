// Answer 0

#[derive(Debug)]
struct MockWriter {
    output: String,
}

impl MockWriter {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

impl std::fmt::Write for MockWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

mod ast {
    #[derive(Debug)]
    pub struct ClassAscii {
        pub kind: ClassAsciiKind,
        pub negated: bool,
    }

    #[derive(Debug)]
    pub enum ClassAsciiKind {
        Upper,
    }
}

#[test]
fn test_fmt_class_ascii_upper() {
    let mut writer = MockWriter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Upper,
        negated: false,
    };
    
    let result = writer.write_str(fmt_class_ascii(&mut writer, &ast).unwrap());
    assert_eq!(writer.output, "[:upper:]");
}

#[test]
fn test_fmt_class_ascii_upper_negated() {
    let mut writer = MockWriter::new();
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Upper,
        negated: true,
    };
    
    let result = writer.write_str(fmt_class_ascii(&mut writer, &ast).unwrap());
    assert_eq!(writer.output, "[:^upper:]");
}

fn fmt_class_ascii(writer: &mut MockWriter, ast: &ast::ClassAscii) -> std::fmt::Result {
    use ast::ClassAsciiKind::*;
    match ast.kind {
        Upper if ast.negated => writer.write_str("[:^upper:]"),
        Upper => writer.write_str("[:upper:]"),
        _ => Ok(()),
    }
}

