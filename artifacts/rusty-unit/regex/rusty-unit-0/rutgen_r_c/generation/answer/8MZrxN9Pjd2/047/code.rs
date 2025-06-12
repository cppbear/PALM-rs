// Answer 0

#[test]
fn test_visit_pre_empty() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::empty();
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_visit_pre_unity_literal_unicode() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::literal(hir::Literal::Unicode('a'));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "a");
}

#[test]
fn test_visit_pre_unity_literal_byte() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::literal(hir::Literal::Byte(97));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "a");
}

#[test]
fn test_visit_pre_class_unicode_single() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_range = ClassUnicodeRange::new('a', 'a');
    let class = ClassUnicode::new(vec![unicode_range]);
    let hir = Hir::class(hir::Class::Unicode(class));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "[a]");
}

#[test]
fn test_visit_pre_class_unicode_multiple() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let unicode_range1 = ClassUnicodeRange::new('a', 'b');
    let unicode_range2 = ClassUnicodeRange::new('d', 'd');
    let class = ClassUnicode::new(vec![unicode_range1, unicode_range2]);
    let hir = Hir::class(hir::Class::Unicode(class));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "[a-b][d]");
}

#[test]
fn test_visit_pre_class_bytes_single() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let byte_range = ClassBytesRange::new(97, 97);
    let class = ClassBytes::new(vec![byte_range]);
    let hir = Hir::class(hir::Class::Bytes(class));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "(?-u:[a])");
}

#[test]
fn test_visit_pre_class_bytes_multiple() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let byte_range1 = ClassBytesRange::new(97, 98);
    let byte_range2 = ClassBytesRange::new(100, 100);
    let class = ClassBytes::new(vec![byte_range1, byte_range2]);
    let hir = Hir::class(hir::Class::Bytes(class));
    assert!(writer.visit_pre(&hir).is_ok());
    assert_eq!(output, "(?-u:[a-b][d])");
}

