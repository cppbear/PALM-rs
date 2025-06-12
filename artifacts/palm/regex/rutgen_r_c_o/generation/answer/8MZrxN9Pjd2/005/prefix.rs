// Answer 0

#[test]
fn test_visit_pre_empty_capture_name() {
    let mut buf = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buf };
    let hir = Hir {
        kind: HirKind::Group(Group {
            kind: hir::GroupKind::CaptureName {
                name: "".to_string(),
                index: None,
            },
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };
    let _ = writer.visit_pre(&hir);
}

#[test]
fn test_visit_pre_non_empty_capture_name() {
    let mut buf = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buf };
    let hir = Hir {
        kind: HirKind::Group(Group {
            kind: hir::GroupKind::CaptureName {
                name: "valid_name".to_string(),
                index: None,
            },
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };
    let _ = writer.visit_pre(&hir);
}

#[should_panic]
#[test]
fn test_visit_pre_invalid_capture_name_format() {
    let mut buf = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buf };
    let hir = Hir {
        kind: HirKind::Group(Group {
            kind: hir::GroupKind::CaptureName {
                name: "invalid_name_format!@#".to_string(),
                index: None,
            },
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };
    let _ = writer.visit_pre(&hir);
}

