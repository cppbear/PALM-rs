// Answer 0

#[test]
fn test_visit_pre_unicode_class_equal_start_end() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let unicode_range = ClassUnicodeRange::new('a', 'a');
    let unicode_class = ClassUnicode::new(vec![unicode_range]);
    let hir = Hir::class(hir::Class::Unicode(unicode_class));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_byte_literal() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let byte_literal = hir::Literal::Byte(255);
    let hir = Hir::literal(byte_literal);
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_bytes_class_with_equal_range() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let byte_range = ClassBytesRange::new(5, 5);
    let bytes_class = ClassBytes::new(vec![byte_range]);
    let hir = Hir::class(hir::Class::Bytes(bytes_class));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

#[test]
#[should_panic]
fn test_visit_pre_byte_class_with_range_err() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let byte_range = ClassBytesRange::new(255, 255);
    let bytes_class = ClassBytes::new(vec![byte_range]);
    let hir = Hir::class(hir::Class::Bytes(bytes_class));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_unicode_class_multiple_ranges() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let unicode_range1 = ClassUnicodeRange::new('a', 'a');
    let unicode_range2 = ClassUnicodeRange::new('b', 'b');
    let unicode_class = ClassUnicode::new(vec![unicode_range1, unicode_range2]);
    let hir = Hir::class(hir::Class::Unicode(unicode_class));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_byte_class_with_different_ranges() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let byte_range = ClassBytesRange::new(1, 2);
    let bytes_class = ClassBytes::new(vec![byte_range]);
    let hir = Hir::class(hir::Class::Bytes(bytes_class));
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.visit_pre(&hir).unwrap();
}

