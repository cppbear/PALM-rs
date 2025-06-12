// Answer 0

#[test]
fn test_fmt_repetition_range_exactly() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: mock_writer,
    };

    let repetition_range = ast::RepetitionRange::Exactly(5);
    
    let result = writer.fmt_repetition_range(&repetition_range);
    
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "{{5}}");
}

#[test]
fn test_fmt_repetition_range_at_least() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: mock_writer,
    };

    let repetition_range = ast::RepetitionRange::AtLeast(3);
    
    let result = writer.fmt_repetition_range(&repetition_range);
    
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "{{3,}}");
}

#[test]
fn test_fmt_repetition_range_bounded() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: mock_writer,
    };

    let repetition_range = ast::RepetitionRange::Bounded(2, 4);
    
    let result = writer.fmt_repetition_range(&repetition_range);
    
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "{{2,4}}");
}

