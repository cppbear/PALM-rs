// Answer 0

#[derive(Default)]
struct TestWriter {
    output: String,
}

impl TestWriter {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.output.push_str(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result<(), std::fmt::Error> {
        self.output.push(c);
        Ok(())
    }
}

#[derive(Debug)]
struct ClassUnicode {
    negated: bool,
    kind: ClassUnicodeKind,
}

#[derive(Debug)]
enum ClassUnicodeKind {
    OneLetter(char),
    Named(String),
    NamedValue { op: ClassUnicodeOpKind, name: String, value: String },
}

#[derive(Debug)]
enum ClassUnicodeOpKind {
    Equal,
    Colon,
    NotEqual,
}

impl TestWriter {
    fn fmt_class_unicode(&mut self, ast: &ClassUnicode) -> Result<(), std::fmt::Error> {
        use ClassUnicodeKind::*;
        use ClassUnicodeOpKind::*;

        if ast.negated {
            self.write_str(r"\P")?;
        } else {
            self.write_str(r"\p")?;
        }
        match ast.kind {
            OneLetter(c) => self.write_char(c),
            Named(ref x) => write!(self, "{{{}}}", x),
            NamedValue { op: Equal, ref name, ref value } => {
                write!(self, "{{{}={}}}", name, value)
            }
            NamedValue { op: Colon, ref name, ref value } => {
                write!(self, "{{{}:{}}}", name, value)
            }
            NamedValue { op: NotEqual, ref name, ref value } => {
                write!(self, "{{{}!={}}}", name, value)
            }
        }
    }
}

#[test]
fn test_fmt_class_unicode_one_letter() {
    let mut writer = TestWriter::default();
    let ast = ClassUnicode {
        negated: false,
        kind: ClassUnicodeKind::OneLetter('a'),
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\pa");
}

#[test]
fn test_fmt_class_unicode_named() {
    let mut writer = TestWriter::default();
    let ast = ClassUnicode {
        negated: false,
        kind: ClassUnicodeKind::Named("Lu".to_string()),
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\p{Lu}");
}

#[test]
fn test_fmt_class_unicode_named_value_equal() {
    let mut writer = TestWriter::default();
    let ast = ClassUnicode {
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::Equal,
            name: "sc".to_string(),
            value: "Latin".to_string(),
        },
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\p{sc=Latin}");
}

#[test]
fn test_fmt_class_unicode_named_value_colon() {
    let mut writer = TestWriter::default();
    let ast = ClassUnicode {
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::Colon,
            name: "sc".to_string(),
            value: "Latin".to_string(),
        },
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\p{sc:Latin}");
}

#[test]
fn test_fmt_class_unicode_named_value_not_equal() {
    let mut writer = TestWriter::default();
    let ast = ClassUnicode {
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: "sc".to_string(),
            value: "Latin".to_string(),
        },
    };
    writer.fmt_class_unicode(&ast).unwrap();
    assert_eq!(writer.output, r"\p{sc!=Latin}");
}

