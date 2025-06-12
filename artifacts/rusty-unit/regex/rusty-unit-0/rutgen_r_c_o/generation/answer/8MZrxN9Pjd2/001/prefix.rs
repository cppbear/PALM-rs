// Answer 0

#[test]
fn test_visit_pre_alternation() {
    let mut printer = Printer { _priv: () };
    let mut writer = vec![];
    let hir = Hir::alternation(vec![
        Hir::literal(hir::Literal::Unicode('a')),
        Hir::literal(hir::Literal::Byte(1)),
    ]);
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_concat() {
    let mut printer = Printer { _priv: () };
    let mut writer = vec![];
    let hir = Hir::concat(vec![
        Hir::literal(hir::Literal::Unicode('b')),
        Hir::literal(hir::Literal::Byte(2)),
    ]);
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_empty() {
    let mut printer = Printer { _priv: () };
    let mut writer = vec![];
    let hir = Hir::empty();
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_repetition() {
    let mut printer = Printer { _priv: () };
    let mut writer = vec![];
    let hir = Hir::repetition(Hir::literal(hir::Literal::Unicode('c')));
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut printer = Printer { _priv: () };
    let mut writer = vec![];
    let unicode_range = ClassUnicodeRange::new('a', 'z');
    let class_unicode = ClassUnicode::new(vec![unicode_range]);
    let hir = Hir::class(hir::Class::Unicode(class_unicode));
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    visitor.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut printer = Printer { _priv: () };
    let mut writer = vec![];
    let byte_range = ClassBytesRange::new(1, 5);
    let class_bytes = ClassBytes::new(vec![byte_range]);
    let hir = Hir::class(hir::Class::Bytes(class_bytes));
    let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
    visitor.visit_pre(&hir).unwrap();
}

