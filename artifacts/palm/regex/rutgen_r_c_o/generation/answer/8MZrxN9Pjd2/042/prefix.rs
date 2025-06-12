// Answer 0

#[test]
fn test_visit_pre_unicode_class() {
    let mut printer = Printer { _priv: () };
    
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };

    let unicode_class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'c')]);
    let hir = Hir::class(Class::Unicode(unicode_class));
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_bytes_class() {
    let mut printer = Printer { _priv: () };

    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };

    let byte_class = ClassBytes::new(vec![ClassBytesRange::new(1, 3)]);
    let hir = Hir::class(Class::Bytes(byte_class));
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_empty() {
    let mut printer = Printer { _priv: () };

    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };

    let hir = Hir::empty();
    visitor.visit_pre(&hir).unwrap();
}

#[test]
#[should_panic]
fn test_visit_pre_unicode_class_with_error() {
    let mut printer = Printer { _priv: () };

    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };

    let unicode_class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
    let hir = Hir::class(Class::Unicode(unicode_class));
    visitor.wtr.write_str("-").unwrap(); // induce an error
    visitor.visit_pre(&hir).unwrap();
}

#[test]
#[should_panic]
fn test_visit_pre_bytes_class_with_error() {
    let mut printer = Printer { _priv: () };

    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };

    let byte_class = ClassBytes::new(vec![ClassBytesRange::new(5, 10)]);
    let hir = Hir::class(Class::Bytes(byte_class));
    visitor.wtr.write_str("-").unwrap(); // induce an error
    visitor.visit_pre(&hir).unwrap();
}

