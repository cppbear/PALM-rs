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

    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.output.push_str(s);
        Ok(())
    }
}

struct MockVisitor<'a> {
    wtr: &'a mut MockWriter,
}

impl<'a> MockVisitor<'a> {
    fn write_literal_char(&mut self, _: char) -> Result<(), std::fmt::Error> {
        Ok(())
    }
    
    fn write_literal_byte(&mut self, _: u8) -> Result<(), std::fmt::Error> {
        Ok(())
    }

    fn write_literal_class_byte(&mut self, _: u8) -> Result<(), std::fmt::Error> {
        Ok(())
    }

    fn visit_pre(&mut self, hir: &Hir) -> std::fmt::Result {
        match *hir.kind() {
            HirKind::Empty
            | HirKind::Repetition(_)
            | HirKind::Concat(_)
            | HirKind::Alternation(_) => {}
            HirKind::Class(hir::Class::Unicode(ref cls)) => {
                self.wtr.write_str("[")?;
                for range in cls.iter() {
                    if range.start() == range.end() {
                        self.write_literal_char(range.start())?;
                    } else {
                        self.write_literal_char(range.start())?;
                        self.wtr.write_str("-")?;
                        self.write_literal_char(range.end())?;
                    }
                }
                self.wtr.write_str("]")?;
            }
            HirKind::Class(hir::Class::Bytes(ref cls)) => {
                self.wtr.write_str("(?-u:[")?;
                for range in cls.iter() {
                    if range.start() == range.end() {
                        self.write_literal_class_byte(range.start())?;
                    } else {
                        self.write_literal_class_byte(range.start())?;
                        self.wtr.write_str("-")?;
                        self.write_literal_class_byte(range.end())?;
                    }
                }
                self.wtr.write_str("])")?;
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Hir {
    kind: HirKind,
}

impl Hir {
    fn kind(&self) -> &HirKind {
        &self.kind
    }
}

#[derive(Debug)]
enum HirKind {
    Empty,
    Repetition(),
    Concat(),
    Alternation(),
    Class(Class),
}

#[derive(Debug)]
enum Class {
    Unicode(Vec<Range>),
    Bytes(Vec<Range>),
}

#[derive(Debug)]
struct Range {
    start: char,
    end: char,
}

impl Range {
    fn start(&self) -> char {
        self.start
    }

    fn end(&self) -> char {
        self.end
    }
}

#[test]
fn test_visit_pre_empty() {
    let mut writer = MockWriter::new();
    let mut visitor = MockVisitor { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Empty,
    };
    assert!(visitor.visit_pre(&hir).is_ok());
}

#[test]
fn test_visit_pre_unicode_class_no_ranges() {
    let mut writer = MockWriter::new();
    let mut visitor = MockVisitor { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Class(Class::Unicode(vec![])),
    };
    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(writer.output, "[");
}

#[test]
fn test_visit_pre_unicode_class_single_char() {
    let mut writer = MockWriter::new();
    let mut visitor = MockVisitor { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Class(Class::Unicode(vec![Range { start: 'a', end: 'a' }])),
    };
    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(writer.output, "[a]");
}

#[test]
fn test_visit_pre_unicode_class_range() {
    let mut writer = MockWriter::new();
    let mut visitor = MockVisitor { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Class(Class::Unicode(vec![Range { start: 'a', end: 'c' }])),
    };
    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(writer.output, "[a-c]");
}

#[test]
fn test_visit_pre_bytes_class_empty() {
    let mut writer = MockWriter::new();
    let mut visitor = MockVisitor { wtr: &mut writer };
    let hir = Hir {
        kind: HirKind::Class(Class::Bytes(vec![])),
    };
    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(writer.output, "(?-u:[");
}

