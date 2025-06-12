// Answer 0

#[test]
fn test_visit_pre_empty_hir() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };

    let hir = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_visit_pre_repetition_hir() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };

    let hir = Hir { kind: HirKind::Repetition(Box::new(Hir::empty())), info: HirInfo::default() };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_visit_pre_concat_hir() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };

    let hir = Hir { kind: HirKind::Concat(vec![Hir::empty(), Hir::empty()]), info: HirInfo::default() };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_visit_pre_alternation_hir() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };

    let hir = Hir { kind: HirKind::Alternation(vec![Hir::empty(), Hir::empty()]), info: HirInfo::default() };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_visit_pre_unicode_literal() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };

    let hir = Hir { kind: HirKind::Literal(hir::Literal::Unicode('a')), info: HirInfo::default() };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "a");
}

#[test]
fn test_visit_pre_byte_literal() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };

    let hir = Hir { kind: HirKind::Literal(hir::Literal::Byte(97)), info: HirInfo::default() };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "a");
}

#[test]
fn test_visit_pre_unicode_class() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };

    let unicode_class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'a')]);
    let hir = Hir { kind: HirKind::Class(hir::Class::Unicode(unicode_class)), info: HirInfo::default() };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "[a]");
}

#[test]
fn test_visit_pre_bytes_class() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };

    let bytes_class = ClassBytes::new(vec![ClassBytesRange::new(97, 97)]);
    let hir = Hir { kind: HirKind::Class(hir::Class::Bytes(bytes_class)), info: HirInfo::default() };
    
    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(output, "(-u:[a])");
}

