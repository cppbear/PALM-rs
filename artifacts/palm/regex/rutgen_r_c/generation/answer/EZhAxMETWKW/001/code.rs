// Answer 0

#[test]
fn test_fmt_repetition_range_exactly() {
    use std::fmt::Write;
    struct TestPrinter {
        output: String,
    }
    
    impl fmt::Write for TestPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut test_printer = TestPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: test_printer };
    
    let ast = ast::RepetitionRange::Exactly(3);
    writer.fmt_repetition_range(&ast).unwrap();
    
    assert_eq!(writer.wtr.output, "{{3}}");
}

#[test]
fn test_fmt_repetition_range_at_least() {
    use std::fmt::Write;
    struct TestPrinter {
        output: String,
    }
    
    impl fmt::Write for TestPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut test_printer = TestPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: test_printer };
    
    let ast = ast::RepetitionRange::AtLeast(2);
    writer.fmt_repetition_range(&ast).unwrap();
    
    assert_eq!(writer.wtr.output, "{{2,}}");
}

#[test]
fn test_fmt_repetition_range_bounded() {
    use std::fmt::Write;
    struct TestPrinter {
        output: String,
    }
    
    impl fmt::Write for TestPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut test_printer = TestPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: test_printer };
    
    let ast = ast::RepetitionRange::Bounded(1, 5);
    writer.fmt_repetition_range(&ast).unwrap();
    
    assert_eq!(writer.wtr.output, "{{1,5}}");
}

