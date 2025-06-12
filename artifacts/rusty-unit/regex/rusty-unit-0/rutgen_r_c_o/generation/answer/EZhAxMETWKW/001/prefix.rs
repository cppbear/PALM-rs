// Answer 0

#[test]
fn test_fmt_repetition_range_bounded_min() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::Bounded(0, 0);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded_small() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::Bounded(1, 5);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded_large() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::Bounded(10, 100);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded_max() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::Bounded(u32::MAX - 1, u32::MAX);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded_edge() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::RepetitionRange::Bounded(u32::MAX, u32::MAX);
    writer.fmt_repetition_range(&ast).unwrap();
}

