// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut printer = Printer { _priv: () };
    let hir = Hir::empty();
    let mut writer = Writer { printer: &mut printer, wtr: Vec::new() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_unicode() {
    let mut printer = Printer { _priv: () };
    let hir = Hir::literal(hir::Literal::Unicode('a'));
    let mut writer = Writer { printer: &mut printer, wtr: Vec::new() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_byte() {
    let mut printer = Printer { _priv: () };
    let hir = Hir::literal(hir::Literal::Byte(65));
    let mut writer = Writer { printer: &mut printer, wtr: Vec::new() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_unicode_class() {
    let mut printer = Printer { _priv: () };
    let mut unicode_class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'b')]);
    let hir = Hir::class(hir::Class::Unicode(unicode_class));
    let mut writer = Writer { printer: &mut printer, wtr: Vec::new() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_bytes_class() {
    let mut printer = Printer { _priv: () };
    let mut bytes_class = ClassBytes::new(vec![ClassBytesRange::new(0, 255)]);
    let hir = Hir::class(hir::Class::Bytes(bytes_class));
    let mut writer = Writer { printer: &mut printer, wtr: Vec::new() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_unicode_negate() {
    let mut printer = Printer { _priv: () };
    let hir = Hir::word_boundary(hir::WordBoundary::UnicodeNegate);
    let mut writer = Writer { printer: &mut printer, wtr: Vec::new() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_ascii() {
    let mut printer = Printer { _priv: () };
    let hir = Hir::word_boundary(hir::WordBoundary::Ascii);
    let mut writer = Writer { printer: &mut printer, wtr: Vec::new() };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_word_boundary_ascii_negate() {
    let mut printer = Printer { _priv: () };
    let hir = Hir::word_boundary(hir::WordBoundary::AsciiNegate);
    let mut writer = Writer { printer: &mut printer, wtr: Vec::new() };
    writer.visit_pre(&hir).unwrap();
}

