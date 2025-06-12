// Answer 0

#[test]
fn test_fmt_class_perl_digit_negated() {
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
    let class_perl = ast::ClassPerl {
        span: Span::default(), // assuming Span has a default implementation
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    writer_instance.fmt_class_perl(&class_perl);
}

#[test]
fn test_fmt_class_perl_digit_non_negated() {
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
    let class_perl = ast::ClassPerl {
        span: Span::default(), // assuming Span has a default implementation
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    writer_instance.fmt_class_perl(&class_perl);
}

#[test]
fn test_fmt_class_perl_space_negated() {
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
    let class_perl = ast::ClassPerl {
        span: Span::default(), // assuming Span has a default implementation
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    writer_instance.fmt_class_perl(&class_perl);
}

#[test]
fn test_fmt_class_perl_space_non_negated() {
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
    let class_perl = ast::ClassPerl {
        span: Span::default(), // assuming Span has a default implementation
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    writer_instance.fmt_class_perl(&class_perl);
}

#[test]
fn test_fmt_class_perl_word_negated() {
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
    let class_perl = ast::ClassPerl {
        span: Span::default(), // assuming Span has a default implementation
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    writer_instance.fmt_class_perl(&class_perl);
}

#[test]
fn test_fmt_class_perl_word_non_negated() {
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
    let class_perl = ast::ClassPerl {
        span: Span::default(), // assuming Span has a default implementation
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    writer_instance.fmt_class_perl(&class_perl);
}

