// Answer 0

#[test]
fn test_visit_pre_literal_unicode() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let hir = Hir::literal(hir::Literal::Unicode('a'));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_literal_byte() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let hir = Hir::literal(hir::Literal::Byte(10));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let unicode_range = ClassUnicodeRange::new('a', 'c');
    let class_unicode = ClassUnicode::new(vec![unicode_range]);

    let hir = Hir::class(hir::Class::Unicode(class_unicode));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let bytes_range_1 = ClassBytesRange::new(10, 20);
    let bytes_range_2 = ClassBytesRange::new(30, 40);
    let class_bytes = ClassBytes::new(vec![bytes_range_1, bytes_range_2]);

    let hir = Hir::class(hir::Class::Bytes(class_bytes));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_unicode_multi_range() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let unicode_range_1 = ClassUnicodeRange::new('a', 'a');
    let unicode_range_2 = ClassUnicodeRange::new('d', 'f');
    let class_unicode = ClassUnicode::new(vec![unicode_range_1, unicode_range_2]);

    let hir = Hir::class(hir::Class::Unicode(class_unicode));
    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_class_bytes_multi_range() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let bytes_range_1 = ClassBytesRange::new(1, 1);
    let bytes_range_2 = ClassBytesRange::new(5, 10);
    let class_bytes = ClassBytes::new(vec![bytes_range_1, bytes_range_2]);

    let hir = Hir::class(hir::Class::Bytes(class_bytes));
    writer.visit_pre(&hir).unwrap();
}

