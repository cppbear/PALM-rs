// Answer 0

#[test]
fn test_fmt_repetition_range_at_least_zero() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::AtLeast(0);
    writer.fmt_repetition_range(&ast);
}

#[test]
fn test_fmt_repetition_range_at_least_fifty() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::AtLeast(50);
    writer.fmt_repetition_range(&ast);
}

#[test]
fn test_fmt_repetition_range_at_least_one_hundred() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::AtLeast(100);
    writer.fmt_repetition_range(&ast);
}

#[test]
fn test_fmt_repetition_range_exactly_zero() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::Exactly(0);
    writer.fmt_repetition_range(&ast);
}

#[test]
fn test_fmt_repetition_range_bounded() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::Bounded(1, 5);
    writer.fmt_repetition_range(&ast);
}

