// Answer 0

#[test]
fn test_fmt_repetition_range_exactly() {
    use std::fmt::Write;
    use ast::RepetitionRange;

    struct MockPrinter {
        output: String,
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer.output };

    let result = writer.fmt_repetition_range(&RepetitionRange::Exactly(3));
    assert!(result.is_ok());
    assert_eq!(printer.output, "{3}");
}

#[test]
fn test_fmt_repetition_range_at_least() {
    use std::fmt::Write;
    use ast::RepetitionRange;

    struct MockPrinter {
        output: String,
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer.output };

    let result = writer.fmt_repetition_range(&RepetitionRange::AtLeast(5));
    assert!(result.is_ok());
    assert_eq!(printer.output, "{5,}");
}

#[test]
fn test_fmt_repetition_range_bounded() {
    use std::fmt::Write;
    use ast::RepetitionRange;

    struct MockPrinter {
        output: String,
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer.output };

    let result = writer.fmt_repetition_range(&RepetitionRange::Bounded(1, 4));
    assert!(result.is_ok());
    assert_eq!(printer.output, "{1,4}");
}

