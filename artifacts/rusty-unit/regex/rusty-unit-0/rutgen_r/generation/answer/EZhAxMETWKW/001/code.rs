// Answer 0

#[test]
fn test_fmt_repetition_range_bounded() {
    use std::fmt;
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    struct TestAst; // Placeholder structure

    let bounded_cases = vec![(1, 3), (2, 5), (0, 10), (10, 10)];

    for (x, y) in bounded_cases {
        let ast = ast::RepetitionRange::Bounded(x, y);
        let result = writer.fmt_repetition_range(&ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, format!("{{{},{{{}}}}}", x, y));
        writer.output.clear(); // Clear the output for next case
    }
}

