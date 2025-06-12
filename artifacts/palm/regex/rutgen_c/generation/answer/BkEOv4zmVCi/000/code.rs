// Answer 0

#[test]
fn test_visit_alternation_in() {
    struct SimpleWriter {
        output: String,
    }

    impl fmt::Write for SimpleWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = SimpleWriter { output: String::new() };
    let printer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut printer, wtr: writer };

    let result = visitor.visit_alternation_in();

    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "|");
}

