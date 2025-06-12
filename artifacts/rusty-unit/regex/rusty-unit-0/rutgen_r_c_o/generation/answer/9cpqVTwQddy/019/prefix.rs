// Answer 0

#[test]
fn test_fmt_class_ascii_cntrl_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Cntrl,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_cntrl_non_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = DummyWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };

    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Cntrl,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast);
}

