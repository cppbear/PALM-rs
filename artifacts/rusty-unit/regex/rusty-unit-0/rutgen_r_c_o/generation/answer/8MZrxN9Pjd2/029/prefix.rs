// Answer 0

#[test]
fn test_visit_pre_with_empty_class_bytes() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let cls = ClassBytes::new(vec![ClassBytesRange::new(0, 0)]);
    let hir = Hir::class(hir::Class::Bytes(cls));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_with_single_literal_byte() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir::literal(hir::Literal::Byte(65)); // ASCII 'A'
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_with_range_class_bytes() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let cls = ClassBytes::new(vec![ClassBytesRange::new(0, 255)]);
    let hir = Hir::class(hir::Class::Bytes(cls));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_with_empty_class_unicode() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let cls = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'a')]);
    let hir = Hir::class(hir::Class::Unicode(cls));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_with_single_literal_unicode() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let hir = Hir::literal(hir::Literal::Unicode('á'));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_with_range_class_unicode() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let cls = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'b')]);
    let hir = Hir::class(hir::Class::Unicode(cls));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    writer.visit_pre(&hir).unwrap();
}

