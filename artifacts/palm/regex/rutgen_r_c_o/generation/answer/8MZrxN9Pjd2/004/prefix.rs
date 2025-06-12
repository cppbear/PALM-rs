// Answer 0

#[test]
fn test_visit_pre_non_capturing_group() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let empty_hir = Hir::empty();
    let group = Group { kind: hir::GroupKind::NonCapturing, hir: Box::new(empty_hir) };
    let hir = Hir::group(group);

    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_non_capturing_group_with_empty_hir() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let empty_hir = Hir::empty();
    let group = Group { kind: hir::GroupKind::NonCapturing, hir: Box::new(empty_hir) };
    let hir = Hir::group(group);

    writer.visit_pre(&hir).unwrap();
}

#[test]
fn test_visit_pre_non_capturing_group_with_info() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let info = HirInfo::default();
    let empty_hir = Hir { kind: HirKind::Empty, info };
    let group = Group { kind: hir::GroupKind::NonCapturing, hir: Box::new(empty_hir) };
    let hir = Hir::group(group);

    writer.visit_pre(&hir).unwrap();
}

