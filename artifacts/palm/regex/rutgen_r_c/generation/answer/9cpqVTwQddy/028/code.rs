// Answer 0

fn test_fmt_class_ascii_alnum_not_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter {
        output: String::new(),
    };

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    };

    let mut fmt_writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    fmt_writer.fmt_class_ascii(&ast).expect("Formatting failed");

    assert_eq!(writer.output, "[:alnum:]");
}

fn test_fmt_class_ascii_alnum_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter {
        output: String::new(),
    };

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };

    let mut fmt_writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    fmt_writer.fmt_class_ascii(&ast).expect("Formatting failed");

    assert_eq!(writer.output, "[:^alnum:]");
}

fn test_fmt_class_ascii_alpha_not_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter {
        output: String::new(),
    };

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };

    let mut fmt_writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    fmt_writer.fmt_class_ascii(&ast).expect("Formatting failed");

    assert_eq!(writer.output, "[:alpha:]");
}

fn test_fmt_class_ascii_alpha_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter {
        output: String::new(),
    };

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };

    let mut fmt_writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    fmt_writer.fmt_class_ascii(&ast).expect("Formatting failed");

    assert_eq!(writer.output, "[:^alpha:]");
}

// Other test functions can be defined similarly for other ClassAsciiKind variants.

