// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::empty();
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_repetition() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::repetition(vec![Hir::literal(hir::Literal::Unicode('a'))]);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_concat() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::concat(vec![
        Hir::literal(hir::Literal::Unicode('a')),
        Hir::literal(hir::Literal::Unicode('b')),
    ]);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_alternation() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::alternation(vec![
        Hir::literal(hir::Literal::Unicode('a')),
        Hir::literal(hir::Literal::Unicode('b')),
    ]);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_unicode() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::literal(hir::Literal::Unicode('z'));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let cls_unicode = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
    let hir = Hir::class(hir::Class::Unicode(cls_unicode));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let cls_bytes = ClassBytes::new(vec![ClassBytesRange::new(65, 90)]);
    let hir = Hir::class(hir::Class::Bytes(cls_bytes));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::anchor(hir::Anchor::StartLine);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::anchor(hir::Anchor::EndLine);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_unicode() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let hir = Hir::word_boundary(hir::WordBoundary::Unicode);
    writer.visit_pre(&hir).unwrap();
}

