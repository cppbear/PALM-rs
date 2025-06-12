// Answer 0

#[test]
fn test_fmt_repetition_range_at_least() {
    use std::fmt;
    use std::fmt::Write;

    struct TestWriter {
        wtr: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.wtr.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { wtr: String::new() };
    
    let at_least_1 = ast::RepetitionRange::AtLeast(1);
    let _ = writer.fmt_repetition_range(&at_least_1).expect("Should write successfully");
    assert_eq!(writer.wtr, "{1,}");

    writer.wtr.clear(); // Clear for the next test

    let at_least_5 = ast::RepetitionRange::AtLeast(5);
    let _ = writer.fmt_repetition_range(&at_least_5).expect("Should write successfully");
    assert_eq!(writer.wtr, "{5,}");
    
    writer.wtr.clear(); // Clear for the next test

    let at_least_0 = ast::RepetitionRange::AtLeast(0);
    let _ = writer.fmt_repetition_range(&at_least_0).expect("Should write successfully");
    assert_eq!(writer.wtr, "{0,}");
}

