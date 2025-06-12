// Answer 0

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::StartLine),
        info: HirInfo::default(),
    };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "(?m:^)");
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::EndLine),
        info: HirInfo::default(),
    };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "(?m:$)");
}

#[test]
fn test_visit_pre_anchor_start_text() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::StartText),
        info: HirInfo::default(),
    };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, r"\A");
}

#[test]
fn test_visit_pre_anchor_end_text() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::EndText),
        info: HirInfo::default(),
    };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, r"\z");
}

#[test]
fn test_visit_pre_class_unicode() {
    let mut output = String::new();
    let ranges = vec![
        ClassUnicodeRange::new('a', 'b'),
        ClassUnicodeRange::new('c', 'c')
    ];
    let unicode_class = ClassUnicode::new(ranges);
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(unicode_class)),
        info: HirInfo::default(),
    };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "[a-bc]");
}

#[test]
fn test_visit_pre_class_bytes() {
    let mut output = String::new();
    let ranges = vec![
        ClassBytesRange::new(1, 2),
        ClassBytesRange::new(3, 3)
    ];
    let bytes_class = ClassBytes::new(ranges);
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(bytes_class)),
        info: HirInfo::default(),
    };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "(?-u:[\x01-\x02\x03])");
}

