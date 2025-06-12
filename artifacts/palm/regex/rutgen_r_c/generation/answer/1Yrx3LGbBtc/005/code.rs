// Answer 0

#[test]
fn test_fmt_class_perl_digit_negated() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    
    let class_perl = ClassPerl {
        span: Span::default(), // assuming a default for demonstration
        kind: ClassPerlKind::Digit,
        negated: true,
    };
    
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    writer_instance.fmt_class_perl(&class_perl).unwrap();
    
    assert_eq!(writer_instance.wtr.output, r"\D");
}

#[test]
fn test_fmt_class_perl_digit() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    
    let class_perl = ClassPerl {
        span: Span::default(), // assuming a default for demonstration
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    writer_instance.fmt_class_perl(&class_perl).unwrap();
    
    assert_eq!(writer_instance.wtr.output, r"\d");
}

#[test]
fn test_fmt_class_perl_space_negated() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    
    let class_perl = ClassPerl {
        span: Span::default(), // assuming a default for demonstration
        kind: ClassPerlKind::Space,
        negated: true,
    };
    
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    writer_instance.fmt_class_perl(&class_perl).unwrap();
    
    assert_eq!(writer_instance.wtr.output, r"\S");
}

#[test]
fn test_fmt_class_perl_space() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    
    let class_perl = ClassPerl {
        span: Span::default(), // assuming a default for demonstration
        kind: ClassPerlKind::Space,
        negated: false,
    };
    
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    writer_instance.fmt_class_perl(&class_perl).unwrap();
    
    assert_eq!(writer_instance.wtr.output, r"\s");
}

#[test]
fn test_fmt_class_perl_word_negated() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    
    let class_perl = ClassPerl {
        span: Span::default(), // assuming a default for demonstration
        kind: ClassPerlKind::Word,
        negated: true,
    };
    
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    writer_instance.fmt_class_perl(&class_perl).unwrap();
    
    assert_eq!(writer_instance.wtr.output, r"\W");
}

#[test]
fn test_fmt_class_perl_word() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    
    let class_perl = ClassPerl {
        span: Span::default(), // assuming a default for demonstration
        kind: ClassPerlKind::Word,
        negated: false,
    };
    
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    writer_instance.fmt_class_perl(&class_perl).unwrap();
    
    assert_eq!(writer_instance.wtr.output, r"\w");
}

