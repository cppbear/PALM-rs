// Answer 0

#[test]
fn test_fmt_class_perl_word_negated() {
    use regex_syntax::ast::{ClassPerl, ClassPerlKind};
    use std::fmt::{self, Write};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestFormatter {
        wtr: TestWriter,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {
                wtr: TestWriter::new(),
            }
        }
        
        fn fmt_class_perl(&mut self, ast: &ClassPerl) -> fmt::Result {
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

    let mut formatter = TestFormatter::new();
    let ast = ClassPerl { kind: ClassPerlKind::Word, negated: true };
    formatter.fmt_class_perl(&ast).unwrap();
    
    assert_eq!(formatter.wtr.output, r"\W");
}

#[test]
fn test_fmt_class_perl_word_non_negated() {
    use regex_syntax::ast::{ClassPerl, ClassPerlKind};
    use std::fmt::{self, Write};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestFormatter {
        wtr: TestWriter,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {
                wtr: TestWriter::new(),
            }
        }
        
        fn fmt_class_perl(&mut self, ast: &ClassPerl) -> fmt::Result {
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

    let mut formatter = TestFormatter::new();
    let ast = ClassPerl { kind: ClassPerlKind::Word, negated: false };
    formatter.fmt_class_perl(&ast).unwrap();
    
    assert_eq!(formatter.wtr.output, r"\w");
}

