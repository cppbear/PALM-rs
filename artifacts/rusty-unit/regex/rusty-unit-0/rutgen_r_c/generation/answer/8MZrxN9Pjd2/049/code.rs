// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::empty();
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_visit_pre_literal_unicode() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::literal(hir::Literal::Unicode('A'));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "A");
}

#[test]
fn test_visit_pre_literal_byte() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let hir = Hir::literal(hir::Literal::Byte(65));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "A");
} 

#[test]
fn test_visit_pre_class_unicode_single() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let unicode_range = ClassUnicodeRange::new('A', 'A');
    let class_unicode = ClassUnicode::new(vec![unicode_range]);
    let hir = Hir::class(hir::Class::Unicode(class_unicode));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "[A]");
}

#[test]
fn test_visit_pre_class_unicode_range() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let unicode_range1 = ClassUnicodeRange::new('A', 'C');
    let unicode_range2 = ClassUnicodeRange::new('E', 'E');
    let class_unicode = ClassUnicode::new(vec![unicode_range1, unicode_range2]);
    let hir = Hir::class(hir::Class::Unicode(class_unicode));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "[A-C][E]");
}

#[test]
fn test_visit_pre_class_bytes_single() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let byte_range = ClassBytesRange::new(65, 65); // ASCII 'A'
    let class_bytes = ClassBytes::new(vec![byte_range]);
    let hir = Hir::class(hir::Class::Bytes(class_bytes));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "(?-u:[A])");
}

#[test]
fn test_visit_pre_class_bytes_range() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    let byte_range1 = ClassBytesRange::new(65, 67); // ASCII 'A' to 'C'
    let byte_range2 = ClassBytesRange::new(69, 69); // ASCII 'E'
    let class_bytes = ClassBytes::new(vec![byte_range1, byte_range2]);
    let hir = Hir::class(hir::Class::Bytes(class_bytes));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "(?-u:[A-C][E])");
}

