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

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

#[derive(Debug)]
struct ClassAscii {
    kind: ClassAsciiKind,
    negated: bool,
}

#[derive(Debug)]
enum ClassAsciiKind {
    Cntrl,
}

struct Formatter {
    wtr: MockWriter,
}

impl Formatter {
    fn new() -> Self {
        Self {
            wtr: MockWriter::new(),
        }
    }

    fn fmt_class_ascii(&mut self, ast: &ClassAscii) -> fmt::Result {
        use ClassAsciiKind::*;
        match ast.kind {
            Cntrl if ast.negated => self.wtr.write_str("[:^cntrl:]"),
            Cntrl => self.wtr.write_str("[:cntrl:]"),
        }
    }
}

#[test]
fn test_fmt_class_ascii_cntrl() {
    let mut formatter = Formatter::new();
    let ast = ClassAscii {
        kind: ClassAsciiKind::Cntrl,
        negated: false,
    };
    formatter.fmt_class_ascii(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "[:cntrl:]");
}

#[test]
fn test_fmt_class_ascii_cntrl_negated() {
    let mut formatter = Formatter::new();
    let ast = ClassAscii {
        kind: ClassAsciiKind::Cntrl,
        negated: true,
    };
    formatter.fmt_class_ascii(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "[:^cntrl:]");
}

