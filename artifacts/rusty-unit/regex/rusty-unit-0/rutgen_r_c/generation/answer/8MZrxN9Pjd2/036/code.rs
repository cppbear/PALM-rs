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
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_pre_literal_unicode() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::literal(hir::Literal::Unicode('a'));
    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "a");
}

#[test]
fn test_visit_pre_literal_byte() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let hir = Hir::literal(hir::Literal::Byte(255));
    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "(?-u:\\xFF)");
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let range = ClassUnicodeRange::new('a', 'z');
    let class = ClassUnicode::new(vec![range.clone()]);
    let hir = Hir::class(hir::Class::Unicode(class));
    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "[a-z]");
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let range = ClassBytesRange::new(0, 255);
    let class = ClassBytes::new(vec![range.clone()]);
    let hir = Hir::class(hir::Class::Bytes(class));
    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "(?-u:[\\x00-\\xFF])");
}

