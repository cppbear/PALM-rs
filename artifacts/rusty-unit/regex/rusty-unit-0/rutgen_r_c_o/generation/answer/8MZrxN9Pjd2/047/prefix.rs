// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut printer = Printer { _priv: () };
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    let hir = Hir::empty();
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_unicode_class_single() {
    let mut printer = Printer { _priv: () };
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    let range = ClassUnicodeRange::new('A', 'A');
    let class_unicode = ClassUnicode::new(vec![range]);
    let hir = Hir::class(Class::Unicode(class_unicode));
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_unicode_class_range() {
    let mut printer = Printer { _priv: () };
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    let range_1 = ClassUnicodeRange::new('A', 'C');
    let range_2 = ClassUnicodeRange::new('E', 'F');
    let class_unicode = ClassUnicode::new(vec![range_1, range_2]);
    let hir = Hir::class(Class::Unicode(class_unicode));
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_bytes_class_single() {
    let mut printer = Printer { _priv: () };
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    let range = ClassBytesRange::new(0, 0);
    let class_bytes = ClassBytes::new(vec![range]);
    let hir = Hir::class(Class::Bytes(class_bytes));
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_bytes_class_range() {
    let mut printer = Printer { _priv: () };
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    let range_1 = ClassBytesRange::new(65, 67);
    let range_2 = ClassBytesRange::new(69, 70);
    let class_bytes = ClassBytes::new(vec![range_1, range_2]);
    let hir = Hir::class(Class::Bytes(class_bytes));
    visitor.visit_pre(&hir).unwrap();
}

