// Answer 0


struct DummyWriter {
    output: String,
}

impl DummyWriter {
    fn new() -> Self {
        DummyWriter { output: String::new() }
    }
}

impl std::fmt::Write for DummyWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

struct Formatter<W> {
    wtr: W,
}

impl<W> Formatter<W> {
    fn new(wtr: W) -> Self {
        Formatter { wtr }
    }
    
    fn fmt_repetition(&mut self, ast: &ast::Repetition) -> std::fmt::Result {
        use ast::RepetitionKind::*;
        match ast.op.kind {
            ZeroOrOne if ast.greedy => self.wtr.write_str("?"),
            ZeroOrOne => self.wtr.write_str("??"),
            ZeroOrMore if ast.greedy => self.wtr.write_str("*"),
            ZeroOrMore => self.wtr.write_str("*?"),
            OneOrMore if ast.greedy => self.wtr.write_str("+"),
            OneOrMore => self.wtr.write_str("+?"),
            Range(ref x) => {
                self.fmt_repetition_range(x)?;
                if !ast.greedy {
                    self.wtr.write_str("?")?;
                }
                Ok(())
            }
        }
    }

    // This function would normally be defined for handling Range
    fn fmt_repetition_range(&mut self, _x: &std::ops::Range<usize>) -> std::fmt::Result {
        self.wtr.write_str("{0,1}")
    }
}

mod ast {
    pub struct Repetition {
        pub op: RepetitionOperator,
        pub greedy: bool,
    }

    pub struct RepetitionOperator {
        pub kind: RepetitionKind,
    }

    pub enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(std::ops::Range<usize>),
    }
}

#[test]
fn test_fmt_repetition_zero_or_one_greedy_false() {
    let mut writer = DummyWriter::new();
    let mut formatter = Formatter::new(&mut writer);
    
    let ast = ast::Repetition {
        op: ast::RepetitionOperator {
            kind: ast::RepetitionKind::ZeroOrOne,
        },
        greedy: false,
    };

    formatter.fmt_repetition(&ast).expect("Writing to dummy writer failed");
    assert_eq!(writer.output, "??");
}

#[test]
fn test_fmt_repetition_zero_or_one_greedy_true() {
    let mut writer = DummyWriter::new();
    let mut formatter = Formatter::new(&mut writer);
    
    let ast = ast::Repetition {
        op: ast::RepetitionOperator {
            kind: ast::RepetitionKind::ZeroOrOne,
        },
        greedy: true,
    };

    formatter.fmt_repetition(&ast).expect("Writing to dummy writer failed");
    assert_eq!(writer.output, "?");
}


