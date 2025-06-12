// Answer 0

#[test]
fn test_fmt_class_perl_digit_negated() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span {}, kind: ClassPerlKind::Digit, negated: true };
    
    fmt_writer.fmt_class_perl(&class_perl).unwrap();
    
    assert_eq!(fmt_writer.wtr.output, r"\D");
}

#[test]
fn test_fmt_class_perl_digit_not_negated() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span {}, kind: ClassPerlKind::Digit, negated: false };

    fmt_writer.fmt_class_perl(&class_perl).unwrap();
    
    assert_eq!(fmt_writer.wtr.output, r"\d");
}

#[test]
fn test_fmt_class_perl_space_negated() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span {}, kind: ClassPerlKind::Space, negated: true };
    
    fmt_writer.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(fmt_writer.wtr.output, r"\S");
}

#[test]
fn test_fmt_class_perl_space_not_negated() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span {}, kind: ClassPerlKind::Space, negated: false };
    
    fmt_writer.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(fmt_writer.wtr.output, r"\s");
}

#[test]
fn test_fmt_class_perl_word_negated() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span {}, kind: ClassPerlKind::Word, negated: true };
    
    fmt_writer.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(fmt_writer.wtr.output, r"\W");
}

#[test]
fn test_fmt_class_perl_word_not_negated() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };

    let class_perl = ClassPerl { span: Span {}, kind: ClassPerlKind::Word, negated: false };
    
    fmt_writer.fmt_class_perl(&class_perl).unwrap();

    assert_eq!(fmt_writer.wtr.output, r"\w");
}

