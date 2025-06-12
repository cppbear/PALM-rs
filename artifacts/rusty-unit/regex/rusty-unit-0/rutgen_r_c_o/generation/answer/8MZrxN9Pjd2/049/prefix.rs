// Answer 0

#[test]
fn test_visit_pre_literal_byte_zero() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::literal(hir::Literal::Byte(0));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_byte_max() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::literal(hir::Literal::Byte(255));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_unicode_min() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::literal(hir::Literal::Unicode('\u{0000}'));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_unicode_max() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::literal(hir::Literal::Unicode('\u{007F}'));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_unicode_middle() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::literal(hir::Literal::Unicode('A'));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(1, 5)]);
    let hir = Hir::class(hir::Class::Bytes(class_bytes));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let class_unicode = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
    let hir = Hir::class(hir::Class::Unicode(class_unicode));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::anchor(hir::Anchor::StartLine);
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::anchor(hir::Anchor::EndLine);
    writer.visit_pre(&hir).unwrap();
}

