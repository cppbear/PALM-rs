// Answer 0

#[test]
fn test_fmt_repetition_range_exactly() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let ast_exactly_0 = ast::RepetitionRange::Exactly(0);
    writer.fmt_repetition_range(&ast_exactly_0).unwrap();
    let ast_exactly_1000 = ast::RepetitionRange::Exactly(1000);
    writer.fmt_repetition_range(&ast_exactly_1000).unwrap();
}

#[test]
fn test_fmt_repetition_range_at_least() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let ast_at_least_0 = ast::RepetitionRange::AtLeast(0);
    writer.fmt_repetition_range(&ast_at_least_0).unwrap();
    let ast_at_least_1000 = ast::RepetitionRange::AtLeast(1000);
    writer.fmt_repetition_range(&ast_at_least_1000).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let ast_bounded_0_0 = ast::RepetitionRange::Bounded(0, 0);
    writer.fmt_repetition_range(&ast_bounded_0_0).unwrap();
    let ast_bounded_1_1 = ast::RepetitionRange::Bounded(1, 1);
    writer.fmt_repetition_range(&ast_bounded_1_1).unwrap();
    let ast_bounded_0_1000 = ast::RepetitionRange::Bounded(0, 1000);
    writer.fmt_repetition_range(&ast_bounded_0_1000).unwrap();
    let ast_bounded_1000_1000 = ast::RepetitionRange::Bounded(1000, 1000);
    writer.fmt_repetition_range(&ast_bounded_1000_1000).unwrap();
    let ast_bounded_1_1000 = ast::RepetitionRange::Bounded(1, 1000);
    writer.fmt_repetition_range(&ast_bounded_1_1000).unwrap();
}

