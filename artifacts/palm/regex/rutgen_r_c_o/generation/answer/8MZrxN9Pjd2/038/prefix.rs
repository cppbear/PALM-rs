// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let hir = Hir::empty();
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_unicode_class_single() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let hir = Hir::class(Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'a')])));
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_bytes_class_single() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(0, 0)])));
    let _ = writer.visit_pre(&hir);
} 

#[test]
fn test_visit_pre_unicode_class_multiple() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let hir = Hir::class(Class::Unicode(ClassUnicode::new(vec![
        ClassUnicodeRange::new('a', 'a'), 
        ClassUnicodeRange::new('b', 'b')
    ])));
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_bytes_class_multiple() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0, 0), 
        ClassBytesRange::new(1, 1)
    ])));
    let _ = writer.visit_pre(&hir);
}

