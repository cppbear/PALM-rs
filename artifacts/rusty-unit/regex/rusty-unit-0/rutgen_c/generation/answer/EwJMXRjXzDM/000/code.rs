// Answer 0

#[test]
fn test_visit_alternation_in() {
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
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: test_writer };

    let result = writer.visit_alternation_in();

    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "|");
}

