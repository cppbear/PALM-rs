// Answer 0

#[test]
fn test_visit_pre_concat() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut test_writer = TestWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut test_writer };

    let hir_concat = Hir::concat(vec![Hir::empty(), Hir::empty()]);
    let result = writer.visit_pre(&hir_concat);

    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_repetition() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut test_writer = TestWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut test_writer };

    let hir_repetition = Hir::repetition(hir::Repetition::zero_or_more(Hir::empty()));
    let result = writer.visit_pre(&hir_repetition);

    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_empty() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut test_writer = TestWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut test_writer };

    let hir_empty = Hir::empty();
    let result = writer.visit_pre(&hir_empty);

    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_alternation() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut test_writer = TestWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut test_writer };

    let hir_alternation = Hir::alternation(vec![Hir::empty(), Hir::empty()]);
    let result = writer.visit_pre(&hir_alternation);

    assert_eq!(result, Ok(()));
}

