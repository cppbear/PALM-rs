// Answer 0

#[test]
fn test_fmt_repetition_range_exactly() {
    use regex_syntax::ast::{self, RepetitionRange};
    use std::fmt::Write;

    struct TestWriter {
        wtr: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { wtr: String::new() }
        }
        
        fn fmt_repetition_range(&mut self, ast: &RepetitionRange) -> std::fmt::Result {
            match *ast {
                RepetitionRange::Exactly(x) => write!(self.wtr, "{{{}}}", x),
                RepetitionRange::AtLeast(x) => write!(self.wtr, "{{{},}}", x),
                RepetitionRange::Bounded(x, y) => write!(self.wtr, "{{{},{}}}", x, y),
            }
        }
    }

    let mut writer = TestWriter::new();
    let ast = RepetitionRange::Exactly(3);
    writer.fmt_repetition_range(&ast).unwrap();
    assert_eq!(writer.wtr, "{{3}}");
}

#[test]
fn test_fmt_repetition_range_at_least() {
    use regex_syntax::ast::{self, RepetitionRange};
    use std::fmt::Write;

    struct TestWriter {
        wtr: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { wtr: String::new() }
        }
        
        fn fmt_repetition_range(&mut self, ast: &RepetitionRange) -> std::fmt::Result {
            match *ast {
                RepetitionRange::Exactly(x) => write!(self.wtr, "{{{}}}", x),
                RepetitionRange::AtLeast(x) => write!(self.wtr, "{{{},}}", x),
                RepetitionRange::Bounded(x, y) => write!(self.wtr, "{{{},{}}}", x, y),
            }
        }
    }

    let mut writer = TestWriter::new();
    let ast = RepetitionRange::AtLeast(5);
    writer.fmt_repetition_range(&ast).unwrap();
    assert_eq!(writer.wtr, "{{5,}}");
}

#[test]
fn test_fmt_repetition_range_bounded() {
    use regex_syntax::ast::{self, RepetitionRange};
    use std::fmt::Write;

    struct TestWriter {
        wtr: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { wtr: String::new() }
        }
        
        fn fmt_repetition_range(&mut self, ast: &RepetitionRange) -> std::fmt::Result {
            match *ast {
                RepetitionRange::Exactly(x) => write!(self.wtr, "{{{}}}", x),
                RepetitionRange::AtLeast(x) => write!(self.wtr, "{{{},}}", x),
                RepetitionRange::Bounded(x, y) => write!(self.wtr, "{{{},{}}}", x, y),
            }
        }
    }

    let mut writer = TestWriter::new();
    let ast = RepetitionRange::Bounded(2, 6);
    writer.fmt_repetition_range(&ast).unwrap();
    assert_eq!(writer.wtr, "{{2,6}}");
}

