// Answer 0

#[test]
fn test_fmt_repetition_range_exactly() {
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
    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: writer,
    };

    let ast = RepetitionRange::Exactly(3);
    writer_instance.fmt_repetition_range(&ast).unwrap();
    assert_eq!(writer_instance.wtr.output, "{3}");
}

#[test]
fn test_fmt_repetition_range_at_least() {
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
    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: writer,
    };

    let ast = RepetitionRange::AtLeast(5);
    writer_instance.fmt_repetition_range(&ast).unwrap();
    assert_eq!(writer_instance.wtr.output, "{5,}");
}

#[test]
fn test_fmt_repetition_range_bounded() {
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
    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: writer,
    };

    let ast = RepetitionRange::Bounded(2, 4);
    writer_instance.fmt_repetition_range(&ast).unwrap();
    assert_eq!(writer_instance.wtr.output, "{2,4}");
}

