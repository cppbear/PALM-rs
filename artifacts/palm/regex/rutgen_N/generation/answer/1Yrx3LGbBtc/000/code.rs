// Answer 0

#[derive(Debug)]
struct MockWriter {
    output: String,
}

impl MockWriter {
    fn new() -> Self {
        MockWriter {
            output: String::new(),
        }
    }

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

struct Formatter {
    wtr: MockWriter,
}

impl Formatter {
    fn new() -> Self {
        Formatter {
            wtr: MockWriter::new(),
        }
    }

    fn fmt_class_perl(&mut self, ast: &ClassPerl) -> std::fmt::Result {
        use ClassPerlKind::*;
        match ast.kind {
            Digit if ast.negated => self.wtr.write_str(r"\D"),
            Digit => self.wtr.write_str(r"\d"),
            Space if ast.negated => self.wtr.write_str(r"\S"),
            Space => self.wtr.write_str(r"\s"),
            Word if ast.negated => self.wtr.write_str(r"\W"),
            Word => self.wtr.write_str(r"\w"),
        }
    }
}

#[derive(Debug)]
enum ClassPerlKind {
    Digit,
    Space,
    Word,
}

#[derive(Debug)]
struct ClassPerl {
    kind: ClassPerlKind,
    negated: bool,
}

#[test]
fn test_fmt_class_perl_digit() {
    let mut formatter = Formatter::new();
    let ast = ClassPerl {
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    formatter.fmt_class_perl(&ast).unwrap();
    assert_eq!(formatter.wtr.output, r"\d");
}

#[test]
fn test_fmt_class_perl_digit_negated() {
    let mut formatter = Formatter::new();
    let ast = ClassPerl {
        kind: ClassPerlKind::Digit,
        negated: true,
    };
    formatter.fmt_class_perl(&ast).unwrap();
    assert_eq!(formatter.wtr.output, r"\D");
}

#[test]
fn test_fmt_class_perl_space() {
    let mut formatter = Formatter::new();
    let ast = ClassPerl {
        kind: ClassPerlKind::Space,
        negated: false,
    };
    formatter.fmt_class_perl(&ast).unwrap();
    assert_eq!(formatter.wtr.output, r"\s");
}

#[test]
fn test_fmt_class_perl_space_negated() {
    let mut formatter = Formatter::new();
    let ast = ClassPerl {
        kind: ClassPerlKind::Space,
        negated: true,
    };
    formatter.fmt_class_perl(&ast).unwrap();
    assert_eq!(formatter.wtr.output, r"\S");
}

#[test]
fn test_fmt_class_perl_word() {
    let mut formatter = Formatter::new();
    let ast = ClassPerl {
        kind: ClassPerlKind::Word,
        negated: false,
    };
    formatter.fmt_class_perl(&ast).unwrap();
    assert_eq!(formatter.wtr.output, r"\w");
}

#[test]
fn test_fmt_class_perl_word_negated() {
    let mut formatter = Formatter::new();
    let ast = ClassPerl {
        kind: ClassPerlKind::Word,
        negated: true,
    };
    formatter.fmt_class_perl(&ast).unwrap();
    assert_eq!(formatter.wtr.output, r"\W");
}

