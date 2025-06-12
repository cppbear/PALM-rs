// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::empty();
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_range = ClassUnicodeRange::new('a', 'z');
    let class_unicode = ClassUnicode::new(vec![unicode_range.clone()]);
    let hir = Hir::class(hir::Class::Unicode(class_unicode));
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "[a-z]");
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let byte_range = ClassBytesRange::new(1, 3);
    let class_bytes = ClassBytes::new(vec![byte_range.clone()]);
    let hir = Hir::class(hir::Class::Bytes(class_bytes));
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "(?-u:[\x01-\x03])");
}

#[test]
#[should_panic]
fn test_visit_pre_class_unicode_write_str_fail() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_range = ClassUnicodeRange::new('a', 'z');
    let class_unicode = ClassUnicode::new(vec![unicode_range.clone()]);
    let hir = Hir::class(hir::Class::Unicode(class_unicode));
    output.push_str("Test"); // Make it fail on write_str
    let _ = writer.visit_pre(&hir);
}

#[test]
#[should_panic]
fn test_visit_pre_class_bytes_write_str_fail() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let byte_range = ClassBytesRange::new(1, 3);
    let class_bytes = ClassBytes::new(vec![byte_range.clone()]);
    let hir = Hir::class(hir::Class::Bytes(class_bytes));
    output.push_str("Test"); // Make it fail on write_str
    let _ = writer.visit_pre(&hir);
}

