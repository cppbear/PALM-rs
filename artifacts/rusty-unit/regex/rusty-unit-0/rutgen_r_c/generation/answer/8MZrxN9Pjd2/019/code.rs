// Answer 0

#[test]
fn test_visit_pre_anchor_start_line() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::StartLine), info: Default::default() };
    
    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "(?m:^)");    
}

#[test]
fn test_visit_pre_anchor_end_line() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::EndLine), info: Default::default() };

    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "(?m:$)");    
}

#[test]
fn test_visit_pre_anchor_start_text() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::StartText), info: Default::default() };

    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(output, r"\A");    
}

#[test]
fn test_visit_pre_anchor_end_text() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::EndText), info: Default::default() };

    let result = writer.visit_pre(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(output, r"\z");    
}

