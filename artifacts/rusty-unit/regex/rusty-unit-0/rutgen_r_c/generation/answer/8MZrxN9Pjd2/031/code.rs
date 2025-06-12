// Answer 0

#[test]
fn test_visit_pre_empty_hir() {
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
fn test_visit_pre_unicode_literal() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::literal(hir::Literal::Unicode('a'));
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "a");
}

#[test]
fn test_visit_pre_byte_literal() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::literal(hir::Literal::Byte(0x20));
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, " ");
}

#[test]
fn test_visit_pre_unicode_class() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let mut unicode_class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
    let hir = Hir::class(hir::Class::Unicode(unicode_class));
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "[a-z]");
}

#[test]
fn test_visit_pre_bytes_class() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let mut bytes_class = ClassBytes::new(vec![ClassBytesRange::new(0x30, 0x39)]);
    let hir = Hir::class(hir::Class::Bytes(bytes_class));
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "(?-u:[0-9])");
}

#[test]
fn test_visit_pre_bytes_class_single_range() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let mut bytes_class = ClassBytes::new(vec![ClassBytesRange::new(0x31, 0x31)]); // '1'
    let hir = Hir::class(hir::Class::Bytes(bytes_class));
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "(?-u:[1])");
}

