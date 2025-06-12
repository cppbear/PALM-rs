// Answer 0

#[test]
fn test_fmt_repetition_range_exactly() -> Result<(), std::fmt::Error> {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter<'a> {
        wtr: &'a mut TestWriter,
    }

    impl<'a> Formatter<'a> {
        fn fmt_repetition_range(&mut self, ast: &ast::RepetitionRange) -> std::fmt::Result {
            use ast::RepetitionRange::*;
            match *ast {
                Exactly(x) => write!(self.wtr, "{{{}}}", x),
                _ => unreachable!(),
            }
        }
    }

    mod ast {
        #[derive(Copy, Clone)]
        pub enum RepetitionRange {
            Exactly(usize),
            AtLeast(usize),
            Bounded(usize, usize),
        }
    }

    let mut writer = TestWriter::new();
    let mut formatter = Formatter { wtr: &mut writer };

    formatter.fmt_repetition_range(&ast::RepetitionRange::Exactly(3))?;
    assert_eq!(writer.output, "{3}");

    formatter.fmt_repetition_range(&ast::RepetitionRange::Exactly(0))?;
    assert_eq!(writer.output, "{3}{0}");

    formatter.fmt_repetition_range(&ast::RepetitionRange::Exactly(10))?;
    assert_eq!(writer.output, "{3}{0}{10}");

    Ok(())
}

